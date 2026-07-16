//! `squeeze` — one intuitive command for the Crush build/run pipeline.
//!
//! `crush-pkg` already does the real work. `squeeze` is a thin composition
//! layer over `crush_pkg::builder::PackageBuilder` (for crush-language
//! build/check) and `crush_pkg::runners::get_runner_for_payload` /
//! `::get_runner` (for run on any capsule type — crush, native, bun/node/
//! deno/python/sona scripts). It does not reimplement compile or run; it
//! composes the existing `crush-pkg` pipeline into one cargo-shaped UX.
//!
//! `crush-pkg build` and `crush-pkg run` are two separate, uncomposed steps
//! today (`crush-pkg run` interprets the entry source directly, never calls
//! `build()`, never writes `target/`). `squeeze` composes them: the no-
//! subcommand default flow checks builds writes `target/` runs, with one
//! status line per stage, while `squeeze build / check / run` stay available
//! individually for when only one step is wanted.
//!
//! M2 (this commit): full cutover (per SQUEEZE-1 ratification). `cmd_run`
//! and the run leg of `cmd_default` route through the `CapsuleRunner`
//! dispatch for every capsule type — so `squeeze run` works on Script and
//! Native capsules, not just crush. `cmd_build` / `cmd_check` keep
//! `PackageBuilder` (crush-source only) because the `.cvm` artifact is
//! crush-language-specific; those subcommands refuse non-crush with a
//! clear "does not apply" message instead of silently mis-routing.
//!
//! Argument forwarding (`squeeze run -- arg1 arg2 ...`) is intentionally
//! NOT implemented in this M2 first cut — `clap` would consume any
//! trailing args as another subcommand without `trailing_var_arg`. Tracked
//! as a SQUEEZE-1 follow-up.
//!
//! #[start_of_lifetime_relevant_to_M2_cutover]

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use crush_pkg::Manifest;
use crush_pkg::builder::PackageBuilder;
use crush_pkg::runners::ExecutionResult;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "squeeze")]
#[command(about = "Build and run Crush packages — one command, sensible defaults")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Crush package
    New {
        name: String,
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Type-check without emitting bytecode (crush-source only)
    Check,
    /// Compile and write target/<name>.cvm + target/<name>.casm.json (crush-source only)
    Build,
    /// Compile and run, without requiring a prior `build`
    Run,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::New { name, dir }) => cmd_new(&name, dir),
        Some(Commands::Check) => cmd_check(),
        Some(Commands::Build) => cmd_build(),
        Some(Commands::Run) => cmd_run(),
        // No subcommand: the intuitive default — check, build, run, in one go.
        None => cmd_default(),
    }
}

/// Load the manifest + the capsule root directory. Capsule-agnostic — does
/// *not* assume a buildable capsule; per-subcommand guards enforce that.
fn load_manifest_and_root() -> anyhow::Result<(Manifest, PathBuf)> {
    let cwd = std::env::current_dir()?;
    let manifest_path = crush_pkg::manifest::manifest_path(&cwd).ok_or_else(|| {
        anyhow::anyhow!(
            "no capsule.toml (or Capsule.toml / crush.toml / Crush.toml) found in {}",
            cwd.display()
        )
    })?;
    let manifest = Manifest::from_file(&manifest_path).context("reading manifest")?;
    let root = manifest_path.parent().unwrap_or(&cwd).to_path_buf();
    Ok((manifest, root))
}

/// `cmd_build` / `cmd_check` are crush-source-specific (they emit the
/// `.cvm` artifact). For Script and Native capsules, refuse with a clear
/// "does not apply" message — never silently mis-route to `PackageBuilder`.
fn require_crush_buildable(manifest: &Manifest) -> anyhow::Result<()> {
    let lang = manifest.capsule.language.as_str();
    let ext = Path::new(&manifest.capsule.entry)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let language_crush = lang == "crush";
    let ext_crush = matches!(ext.as_str(), "crush" | "casm" | "sno");
    let buildable = language_crush || (lang.is_empty() && ext_crush);
    if !buildable {
        let what = if lang.is_empty() {
            format!(
                "(no `language` field set; detected from entry extension `.{ext}`)"
            )
        } else {
            format!("(manifest declares language = {lang})")
        };
        bail!(
            "squeeze build/check do not apply to Script/Native capsules {what}.              Use `squeeze run` (or `crush-pkg` directly) on this capsule."
        );
    }
    Ok(())
}

fn cmd_new(name: &str, dir: Option<PathBuf>) -> anyhow::Result<()> {
    let dir = dir.unwrap_or_else(|| PathBuf::from(name));
    let manifest = crush_pkg::manifest::scaffold_package(&dir, name)?;
    println!(
        "created {} at {}/capsule.toml",
        manifest.capsule.name,
        dir.display()
    );
    Ok(())
}

fn cmd_check() -> anyhow::Result<()> {
    let (manifest, root) = load_manifest_and_root()?;
    require_crush_buildable(&manifest)?;
    let pkg = PackageBuilder::new(manifest, root);
    println!(
        "checking {} v{}",
        pkg.manifest().capsule.name,
        pkg.manifest().capsule.version
    );
    pkg.check()?;
    println!("ok");
    Ok(())
}

fn cmd_build() -> anyhow::Result<()> {
    let (manifest, root) = load_manifest_and_root()?;
    require_crush_buildable(&manifest)?;
    let pkg = PackageBuilder::new(manifest, root);
    println!(
        "building {} v{}",
        pkg.manifest().capsule.name,
        pkg.manifest().capsule.version
    );
    let output = pkg.build()?;
    pkg.write_output(&output)?;
    println!(
        "done: {} function(s), {} byte(s)",
        output.functions.len(),
        output.program.code.len()
    );
    Ok(())
}

/// Dispatch `run` through `crush_pkg::runners::get_runner_for_payload`.
/// Works for crush (CrushRunner via in-process VM), native (NativeRunner
/// via child process), and ScriptRunner dispatch for bun/node/deno/python/
/// sona scripts (each correctly resolving via `buckets` for pinned
/// toolchains when `capsule.runtime_version` is set).
fn cmd_run() -> anyhow::Result<()> {
    let (manifest, root) = load_manifest_and_root()?;
    let entry_path = root.join(&manifest.capsule.entry);
    dispatch_run(&manifest, &entry_path)
}

/// Default flow (`squeeze` with no subcommand): on crush-source capsules,
/// check and build before running (writes `target/`). On Script/Native
/// capsules, skip check/build entirely (they do not apply on those) and
/// run via the runner dispatch.
fn cmd_default() -> anyhow::Result<()> {
    let (manifest, root) = load_manifest_and_root()?;
    let entry_path = root.join(&manifest.capsule.entry);
    let name = manifest.capsule.name.clone();
    let version = manifest.capsule.version.clone();

    if is_crush_source(&manifest, &entry_path) {
        let pkg = PackageBuilder::new(manifest, root);
        println!("checking {name} v{version}");
        pkg.check()?;
        println!("building {name} v{version}");
        let output = pkg.build()?;
        pkg.write_output(&output)?;
        println!(
            "  {} function(s), {} byte(s)",
            output.functions.len(),
            output.program.code.len()
        );
        println!("running {name}");
        dispatch_run(pkg.manifest(), &entry_path)?;
    } else {
        println!("running {name} v{version}");
        dispatch_run(&manifest, &entry_path)?;
    }
    Ok(())
}

fn is_crush_source(manifest: &Manifest, entry_path: &Path) -> bool {
    let lang = manifest.capsule.language.as_str();
    let ext = entry_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    lang == "crush" || (lang.is_empty() && matches!(ext.as_str(), "crush" | "casm" | "sno"))
}

fn dispatch_run(manifest: &Manifest, entry_path: &Path) -> anyhow::Result<()> {
    let runner = crush_pkg::runners::get_runner_for_payload(entry_path, manifest);
    let result = runner.run(manifest, entry_path, &[])?;
    match result {
        ExecutionResult::Process(mut child) => {
            let status = child.wait()?;
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        ExecutionResult::Vm | ExecutionResult::None => {}
    }
    Ok(())
}
