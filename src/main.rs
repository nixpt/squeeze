//! `squeeze` — one intuitive command for the Crush build/run pipeline.
//!
//! `crush-pkg` already does the real work (parse → compile → CVM1 bytecode,
//! via `crush_pkg::builder::PackageBuilder`); squeeze doesn't reimplement
//! any of it. What it adds: `crush-pkg build` and `crush-pkg run` are two
//! separate, uncomposed steps today (`crush-pkg run` interprets the entry
//! source directly — it doesn't call `build()` first, doesn't need
//! `target/` to exist, and doesn't leave build artifacts behind). Running
//! `squeeze` with no subcommand does check → build → write `target/` →
//! run, in one go and with one status line per stage, the way `cargo run`
//! does — while `squeeze build`/`check`/`run` stay available individually
//! for when you want just one step.
//!
//! Scope for this skeleton: `language = "crush"` capsules only, via
//! `PackageBuilder`/`crush_vm::run_with_caps` directly. `crush-pkg`'s CLI
//! `run` command also supports Script (bun/node/deno/python) and Native
//! capsules through the `CapsuleRunner`/`CrushRunner` trait system in
//! `crush_pkg::runners` — squeeze doesn't wire that in yet. Not a silent
//! gap: a non-crush `language` value errors out explicitly rather than
//! being mis-handled.

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use crush_pkg::Manifest;
use crush_pkg::builder::PackageBuilder;
use std::path::PathBuf;

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
    /// Type-check without emitting bytecode
    Check,
    /// Compile and write target/<name>.cvm + target/<name>.casm.json
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

fn load_capsule() -> anyhow::Result<PackageBuilder> {
    let cwd = std::env::current_dir()?;
    let manifest_path = crush_pkg::manifest::manifest_path(&cwd).ok_or_else(|| {
        anyhow::anyhow!(
            "no capsule.toml (or Capsule.toml / crush.toml / Crush.toml) found in {}",
            cwd.display()
        )
    })?;
    let manifest = Manifest::from_file(&manifest_path).context("reading manifest")?;
    require_crush_language(&manifest)?;
    let root = manifest_path.parent().unwrap_or(&cwd).to_path_buf();
    Ok(PackageBuilder::new(manifest, root))
}

/// This skeleton only wires the `PackageBuilder` (crush-language) path.
/// Fail loudly and by name for anything else rather than silently
/// mis-running it — see the module doc's "Scope" note.
fn require_crush_language(manifest: &Manifest) -> anyhow::Result<()> {
    let lang = manifest.capsule.language.as_str();
    if !lang.is_empty() && lang != "crush" {
        bail!(
            "squeeze v0.1 only builds/runs `language = \"crush\"` capsules \
             (found `language = \"{lang}\"`). Script/Native capsule support \
             is a known gap — see the module doc in src/main.rs. Use \
             `crush-pkg` directly for this capsule in the meantime."
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
    let pkg = load_capsule()?;
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
    let pkg = load_capsule()?;
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

fn cmd_run() -> anyhow::Result<()> {
    let pkg = load_capsule()?;
    let output = pkg.build()?;
    let quotas = crush_vm::Quotas::default();
    let result = crush_vm::run_with_caps(&output.program, &quotas, None)
        .map_err(|e| anyhow::anyhow!("runtime error: {e}"))?;
    print!("{}", result.output);
    if !result.halted {
        eprintln!("(program did not halt — quota exceeded)");
    }
    Ok(())
}

fn cmd_default() -> anyhow::Result<()> {
    let pkg = load_capsule()?;
    let name = pkg.manifest().capsule.name.clone();
    let version = pkg.manifest().capsule.version.clone();

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
    let quotas = crush_vm::Quotas::default();
    let result = crush_vm::run_with_caps(&output.program, &quotas, None)
        .map_err(|e| anyhow::anyhow!("runtime error: {e}"))?;
    print!("{}", result.output);
    if !result.halted {
        eprintln!("(program did not halt — quota exceeded)");
    }
    Ok(())
}
