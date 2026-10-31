//! `squeeze` — Crush's cargo: the one command to create, build, run and test
//! Crush programs.
//!
//! squeeze is the user-facing front door; `crush-pkg` (crush-ast) is the
//! engine underneath. Every stage that touches Crush code — manifest parsing,
//! dependency resolution and `capsule.lock`, import roots, compilation, the
//! `.cvm1`/`.casm` writers, the VM runners, the scaffolds — is a `crush_pkg`
//! call. squeeze owns only the UX: the command surface, status lines, exit
//! codes, the test runner loop, `capsule.toml` editing for `add`/`remove`,
//! and the hand-off to `polish` (fmt/lint) and to a build admission wrapper
//! (`SQUEEZE_ADMIT`). See `.dejavue/decisions.md` (SQUEEZE-8).
//!
//! Exit codes: 0 ok, 1 failure, 2 usage error; `run` (and a bare `squeeze`)
//! exits with the program's own `sys.exit(n)` status.

mod admit;
mod edit;
mod status;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, bail};
use clap::{Args, Parser, Subcommand};
use crush_pkg::Manifest;
use crush_pkg::builder::{BuildOutput, PackageBuilder};
use crush_pkg::flow::Buildability;
use crush_pkg::runners::{CrushRunner, ExecutionResult};

use status::status;

#[derive(Parser)]
#[command(name = "squeeze", version)]
#[command(about = "Crush's cargo: create, build, run and test Crush programs")]
#[command(after_help = "With no command, squeeze builds the package and runs it.")]
struct Cli {
    /// Never touch the network: resolve git dependencies from capsule.lock
    /// and the local cache only
    #[arg(long, global = true)]
    offline: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new package in the standard layout
    New {
        /// Package name (also the directory, unless --dir is given)
        name: String,
        /// Directory to create (default: ./NAME)
        #[arg(long, value_name = "DIR")]
        dir: Option<PathBuf>,
        /// A command-line program: `import facet { facet_run }`, facet as a
        /// git dependency
        #[arg(long, conflicts_with = "lib")]
        cli: bool,
        /// A library: src/lib.crush with no `main`
        #[arg(long)]
        lib: bool,
        /// With --cli: depend on a local facet checkout instead of the
        /// published git tag
        #[arg(long, value_name = "DIR", requires = "cli")]
        facet_path: Option<PathBuf>,
    },
    /// Compile the package to target/<name>.cvm1 and target/<name>.casm
    Build(BuildArgs),
    /// Build the package, then run it (arguments after `--` go to the program)
    Run(RunArgs),
    /// Compile without writing target/, and compare capabilities with
    /// [capabilities]
    Check,
    /// Compile and run every tests/*.crush (a test passes when it exits 0)
    Test {
        /// Only run tests whose file name contains this
        filter: Option<String>,
    },
    /// Remove target/
    Clean,
    /// Add (or replace) a dependency in capsule.toml and update capsule.lock
    Add(AddArgs),
    /// Remove a dependency from capsule.toml and capsule.lock
    Remove {
        /// Dependency name
        name: String,
    },
    /// Format the package's .crush files with polish
    Fmt {
        /// Don't write; exit 1 if any file would change
        #[arg(long)]
        check: bool,
    },
    /// Lint the package's .crush files with polish
    Lint,
    /// Internal: compile and run one test file (used by `squeeze test`)
    #[command(name = "__test-one", hide = true)]
    TestOne { file: PathBuf },
}

#[derive(Args, Clone, Default)]
struct BuildArgs {
    /// Write into target/release/ instead of target/. crush-frontend has
    /// no optimisation levels (its optimizer always runs), so the code is
    /// the same; only the output directory differs
    #[arg(long)]
    release: bool,
}

#[derive(Args, Clone, Default)]
struct RunArgs {
    #[command(flatten)]
    build: BuildArgs,
    /// Grant the filesystem capabilities, confined to the current directory
    #[arg(long)]
    fs: bool,
    /// Grant environment-variable access
    #[arg(long)]
    env: bool,
    /// Grant the clock
    #[arg(long)]
    time: bool,
    /// Arguments for the program (`sys.args()`)
    #[arg(last = true)]
    args: Vec<String>,
}

#[derive(Args)]
struct AddArgs {
    /// Dependency name (what the code imports: `-` becomes `_`)
    name: String,
    /// A git repository (anything `git clone` accepts, incl. file://)
    #[arg(
        long,
        value_name = "URL",
        conflicts_with = "path",
        required_unless_present = "path"
    )]
    git: Option<String>,
    #[arg(long, requires = "git", conflicts_with_all = ["rev", "branch"])]
    tag: Option<String>,
    #[arg(long, requires = "git", conflicts_with = "branch")]
    rev: Option<String>,
    #[arg(long, requires = "git")]
    branch: Option<String>,
    /// A local directory holding the dependency's capsule.toml
    #[arg(long, value_name = "DIR")]
    path: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Some(code) =
        admit::maybe_reexec(cli.command.as_ref().is_none_or(Commands::is_build_work))
    {
        return code;
    }
    match dispatch(cli) {
        Ok(code) => code,
        Err(e) => {
            status::error(&format!("{e:#}"));
            ExitCode::FAILURE
        }
    }
}

impl Commands {
    /// The commands that compile or run Crush code: what `SQUEEZE_ADMIT`
    /// wraps. (Bare `squeeze` = build + run counts too.)
    fn is_build_work(&self) -> bool {
        matches!(
            self,
            Commands::Build(_) | Commands::Run(_) | Commands::Check | Commands::Test { .. }
        )
    }
}

fn dispatch(cli: Cli) -> anyhow::Result<ExitCode> {
    let offline = cli.offline || std::env::var_os("CRUSH_PKG_OFFLINE").is_some_and(|v| v == "1");
    match cli.command {
        Some(Commands::New {
            name,
            dir,
            cli,
            lib,
            facet_path,
        }) => cmd_new(&name, dir, cli, lib, facet_path),
        Some(Commands::Build(b)) => cmd_build(&b, offline).map(|_| ExitCode::SUCCESS),
        Some(Commands::Run(r)) => cmd_run(&r, offline),
        Some(Commands::Check) => cmd_check(offline),
        Some(Commands::Test { filter }) => cmd_test(filter.as_deref(), offline),
        Some(Commands::Clean) => cmd_clean(),
        Some(Commands::Add(a)) => cmd_add(a, offline),
        Some(Commands::Remove { name }) => cmd_remove(&name),
        Some(Commands::Fmt { check }) => {
            cmd_polish(if check { &["fmt", "--check"] } else { &["fmt"] })
        }
        Some(Commands::Lint) => cmd_polish(&["lint"]),
        Some(Commands::TestOne { file }) => cmd_test_one(&file, offline),
        None => cmd_run(&RunArgs::default(), offline),
    }
}

/// The package around the current directory: the nearest `capsule.toml`
/// (or `Capsule.toml` / `crush.toml` / `Crush.toml`) here or in a parent.
fn load_package() -> anyhow::Result<(Manifest, PathBuf, PathBuf)> {
    let cwd = std::env::current_dir()?;
    let Some(manifest_path) = cwd.ancestors().find_map(crush_pkg::manifest::manifest_path) else {
        bail!(
            "could not find capsule.toml in {} or any parent directory\n  \
             (start a package with `squeeze new NAME`)",
            cwd.display()
        );
    };
    let manifest = Manifest::from_file(&manifest_path)
        .with_context(|| format!("reading {}", manifest_path.display()))?;
    let root = manifest_path.parent().unwrap_or(&cwd).to_path_buf();
    Ok((manifest, root, manifest_path))
}

fn cmd_new(
    name: &str,
    dir: Option<PathBuf>,
    cli: bool,
    lib: bool,
    facet_path: Option<PathBuf>,
) -> anyhow::Result<ExitCode> {
    let dir = dir.unwrap_or_else(|| PathBuf::from(name));
    if dir.exists() {
        bail!("destination `{}` already exists", dir.display());
    }
    let kind = if cli {
        let facet = match facet_path {
            Some(p) => crush_pkg::manifest::FacetSource::Path(p),
            None => crush_pkg::manifest::FacetSource::Published,
        };
        crush_pkg::manifest::scaffold_cli_package(&dir, name, &facet)?;
        "command-line program"
    } else if lib {
        crush_pkg::manifest::scaffold_lib_package(&dir, name)?;
        "library"
    } else {
        crush_pkg::manifest::scaffold_package(&dir, name)?;
        "program"
    };
    status(
        "Created",
        &format!("{kind} `{name}` package at {}", dir.display()),
    );
    Ok(ExitCode::SUCCESS)
}

/// A built package: its output, and where the `.cvm1` was written.
struct Built {
    output: BuildOutput,
    cvm1: PathBuf,
}

fn profile_dir(root: &Path, b: &BuildArgs) -> PathBuf {
    if b.release {
        root.join("target").join("release")
    } else {
        root.join("target")
    }
}

fn cmd_build(b: &BuildArgs, offline: bool) -> anyhow::Result<Built> {
    let (manifest, root, _) = load_package()?;
    build_package(manifest, root, b, offline)
}

fn build_package(
    manifest: Manifest,
    root: PathBuf,
    b: &BuildArgs,
    offline: bool,
) -> anyhow::Result<Built> {
    crush_pkg::flow::require_crush_buildable(&manifest, &root, "build").map_err(|_| {
        anyhow::anyhow!("`squeeze build` only applies to Crush-source packages; use `squeeze run`")
    })?;
    let label = format!(
        "{} v{} ({})",
        manifest.capsule.name,
        manifest.capsule.version,
        root.display()
    );
    let name = manifest.capsule.name.clone();
    status("Compiling", &label);
    let builder = PackageBuilder::new(manifest, root.clone())
        .offline(offline)
        .quiet(true);
    let output = builder.build()?;
    for w in &output.program.warnings {
        status::warning(w);
    }
    let dir = profile_dir(&root, b);
    builder.write_output_to(&output, &dir)?;
    let cvm1 = dir.join(format!("{name}.cvm1"));
    let profile = if b.release { "release" } else { "debug" };
    status(
        "Finished",
        &format!(
            "`{profile}` profile: {} ({} function(s), {} byte(s))",
            rel(&root, &cvm1),
            output.functions.len(),
            output.program.code.len()
        ),
    );
    Ok(Built { output, cvm1 })
}

fn runner(manifest: &Manifest, r: &RunArgs) -> CrushRunner {
    let caps = crush_lang_sdk::HostCapsBuilder::new()
        .fs(r.fs)
        .fs_root(".")
        .env(r.env)
        .time(r.time)
        .stdlib(true)
        .args(r.args.clone())
        .build();
    let mut runner = CrushRunner {
        host_caps: Some(caps),
        ..CrushRunner::default()
    };
    if let Some(gas) = manifest.resources.gas {
        runner.quotas.max_steps = usize::try_from(gas).unwrap_or(usize::MAX);
    }
    runner
}

fn cmd_run(r: &RunArgs, offline: bool) -> anyhow::Result<ExitCode> {
    let (manifest, root, _) = load_package()?;
    if let Buildability::NotCrush(_) = crush_pkg::flow::buildability(&manifest, &root) {
        // Script / native capsules: nothing to build; crush-pkg's runner
        // dispatch runs them (bun/node/deno/python/sona/native).
        let entry = root.join(&manifest.capsule.entry);
        status(
            "Running",
            &format!("{} ({})", manifest.capsule.entry, manifest.capsule.language),
        );
        let runner = crush_pkg::runners::get_runner_for_payload(&entry, &manifest);
        return finish(runner.run(&manifest, &entry, &r.args)?);
    }
    let crush_runner = runner(&manifest, r);
    let built = build_package(manifest, root.clone(), &r.build, offline)?;
    let shown = rel(&root, &built.cvm1);
    let mut line = format!("`{shown}`");
    if !r.args.is_empty() {
        line = format!("`{shown} {}`", r.args.join(" "));
    }
    status("Running", &line);
    finish(crush_runner.run_program(&built.output.program)?)
}

/// Turn how a run ended into squeeze's exit status: the program's own
/// `sys.exit(n)` (or a child process's status) passes through.
fn finish(result: ExecutionResult) -> anyhow::Result<ExitCode> {
    use std::io::Write;
    let _ = std::io::stdout().flush();
    Ok(match result {
        ExecutionResult::Vm | ExecutionResult::None => ExitCode::SUCCESS,
        ExecutionResult::Exit(code) => exit_code(code),
        ExecutionResult::Process(mut child) => {
            let st = child.wait()?;
            exit_code(st.code().unwrap_or(1))
        }
    })
}

fn exit_code(code: i32) -> ExitCode {
    // Exit statuses are 0..=255 on unix; out-of-range values wrap the way the
    // OS would truncate them.
    ExitCode::from((code & 0xff) as u8)
}

fn cmd_check(offline: bool) -> anyhow::Result<ExitCode> {
    let (manifest, root, _) = load_package()?;
    crush_pkg::flow::require_crush_buildable(&manifest, &root, "check")
        .map_err(|_| anyhow::anyhow!("`squeeze check` only applies to Crush-source packages"))?;
    status(
        "Checking",
        &format!(
            "{} v{} ({})",
            manifest.capsule.name,
            manifest.capsule.version,
            root.display()
        ),
    );
    let report = PackageBuilder::new(manifest, root)
        .offline(offline)
        .quiet(true)
        .check()?;
    let mut errors = 0;
    for f in &report.findings {
        let line = format!("capsule.toml: {} ({})", f.message, f.hint);
        match f.level {
            crush_pkg::capcheck::Level::Error => {
                errors += 1;
                status::error(&line)
            }
            crush_pkg::capcheck::Level::Warning => status::warning(&line),
        }
    }
    if errors > 0 {
        bail!(
            "{errors} capability error(s): the program uses capabilities capsule.toml doesn't declare"
        );
    }
    let used: Vec<&str> = report.used.iter().map(String::as_str).collect();
    status(
        "Finished",
        &format!(
            "check; capabilities used: {}",
            if used.is_empty() {
                "none".into()
            } else {
                used.join(", ")
            }
        ),
    );
    Ok(ExitCode::SUCCESS)
}

/// `squeeze test`: every `tests/*.crush`, sorted, each compiled with the
/// package's imports (`src/`, the package by name, its dependencies) and
/// run in its own `squeeze __test-one` process, so a test's output is
/// captured and shown only when it fails, and one test can't take the
/// others down.
fn cmd_test(filter: Option<&str>, offline: bool) -> anyhow::Result<ExitCode> {
    let (manifest, root, _) = load_package()?;
    let tests_dir = root.join("tests");
    let mut files: Vec<PathBuf> = match std::fs::read_dir(&tests_dir) {
        Ok(rd) => rd
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "crush") && p.is_file())
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();
    if let Some(f) = filter {
        files.retain(|p| {
            p.file_name()
                .is_some_and(|n| n.to_string_lossy().contains(f))
        });
    }
    status(
        "Testing",
        &format!(
            "{} v{} ({})",
            manifest.capsule.name,
            manifest.capsule.version,
            root.display()
        ),
    );
    // Resolve (and, once, fetch) dependencies up front so every test process
    // can stay offline and they don't race on the cache or capsule.lock.
    PackageBuilder::new(manifest, root.clone())
        .offline(offline)
        .quiet(true)
        .resolve()?;
    println!(
        "\nrunning {} test{}",
        files.len(),
        if files.len() == 1 { "" } else { "s" }
    );
    let exe = std::env::current_exe().context("locating the squeeze executable")?;
    let mut failed: Vec<(String, String)> = Vec::new();
    for file in &files {
        let shown = rel(&root, file);
        let out = std::process::Command::new(&exe)
            .args(["--offline", "__test-one"])
            .arg(file)
            .current_dir(&root)
            .env_remove(admit::ENV_ADMIT)
            .stdin(std::process::Stdio::null())
            .output()
            .with_context(|| format!("running {shown}"))?;
        if out.status.success() {
            println!("test {shown} ... ok");
        } else {
            println!("test {shown} ... FAILED");
            let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&out.stderr));
            let code = out
                .status
                .code()
                .map_or("a signal".to_string(), |c| format!("status {c}"));
            failed.push((shown, format!("{text}(exited with {code})")));
        }
    }
    if !failed.is_empty() {
        println!("\nfailures:");
        for (name, text) in &failed {
            println!("\n---- {name} ----\n{}", text.trim_end());
        }
    }
    let passed = files.len() - failed.len();
    let verdict = if failed.is_empty() { "ok" } else { "FAILED" };
    println!(
        "\ntest result: {verdict}. {passed} passed; {} failed\n",
        failed.len()
    );
    Ok(if failed.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn cmd_test_one(file: &Path, offline: bool) -> anyhow::Result<ExitCode> {
    let (manifest, root, _) = load_package()?;
    let r = RunArgs::default();
    let crush_runner = runner(&manifest, &r);
    let builder = PackageBuilder::new(manifest, root)
        .offline(offline)
        .quiet(true);
    let output = builder.build_file(file)?;
    finish(crush_runner.run_program(&output.program)?)
}

fn cmd_clean() -> anyhow::Result<ExitCode> {
    let (_, root, _) = load_package()?;
    let target = root.join("target");
    if target.exists() {
        std::fs::remove_dir_all(&target)
            .with_context(|| format!("removing {}", target.display()))?;
        status("Removed", &rel(&root, &target));
    }
    Ok(ExitCode::SUCCESS)
}

fn cmd_add(a: AddArgs, offline: bool) -> anyhow::Result<ExitCode> {
    let (manifest, root, manifest_path) = load_package()?;
    if crush_pkg::builder::import_name(&a.name)
        == crush_pkg::builder::import_name(&manifest.capsule.name)
    {
        bail!(
            "`{}` is this package's own name; a package can't depend on itself",
            a.name
        );
    }
    let dep = crush_pkg::manifest::Dependency {
        name: a.name.clone(),
        path: a.path.as_ref().map(|p| edit::relative_path(&root, p)),
        git: a.git.clone(),
        tag: a.tag.clone(),
        rev: a.rev.clone(),
        branch: a.branch.clone(),
        ..Default::default()
    };
    dep.validate()?;
    let what = if let Some(p) = &dep.path {
        let dir = root.join(p);
        if crush_pkg::manifest::manifest_path(&dir).is_none() {
            bail!("--path {}: no capsule.toml there", dir.display());
        }
        if std::fs::canonicalize(&dir).ok() == std::fs::canonicalize(&root).ok() {
            bail!(
                "--path {}: that is this package; a package can't depend on itself",
                dir.display()
            );
        }
        format!("path {p}")
    } else {
        let reference = crush_pkg::gitdep::ref_string(&dep)?;
        format!(
            "{} {}",
            dep.git.as_deref().unwrap_or_default(),
            reference.replacen(':', " ", 1)
        )
    };
    // Resolve before touching any file, so a bad tag or URL changes nothing.
    let lock_entry = if dep.git.is_some() {
        status("Updating", dep.git.as_deref().unwrap_or_default());
        let (_, entry) = crush_pkg::gitdep::resolve(&dep, None, offline)?;
        Some(entry)
    } else {
        None
    };
    let replaced = edit::upsert_dependency(&manifest_path, &dep)?;
    let mut lock = crush_pkg::gitdep::read_lock(&root)?;
    lock.retain(|e| e.name != dep.name);
    if let Some(entry) = &lock_entry {
        lock.push(entry.clone());
    }
    crush_pkg::gitdep::write_lock(&root, &lock)?;
    let pinned = lock_entry
        .map(|e| format!(" @ {}", &e.rev[..12]))
        .unwrap_or_default();
    let verb = if replaced { "Replacing" } else { "Adding" };
    status(verb, &format!("{} ({what}){pinned}", dep.name));
    let import = crush_pkg::builder::import_name(&dep.name);
    status(
        "Note",
        &format!("use it with `import {import}` (or `import {import} {{ fn_name }}`)"),
    );
    Ok(ExitCode::SUCCESS)
}

fn cmd_remove(name: &str) -> anyhow::Result<ExitCode> {
    let (_, root, manifest_path) = load_package()?;
    if !edit::remove_dependency(&manifest_path, name)? {
        bail!("dependency `{name}` is not in {}", manifest_path.display());
    }
    let mut lock = crush_pkg::gitdep::read_lock(&root)?;
    lock.retain(|e| e.name != name);
    crush_pkg::gitdep::write_lock(&root, &lock)?;
    status("Removing", name);
    Ok(ExitCode::SUCCESS)
}

/// `squeeze fmt` / `squeeze lint`: hand the package's `.crush` files (under
/// `src/` and `tests/`) to polish — `$SQUEEZE_POLISH`, else `polish` on PATH.
fn cmd_polish(args: &[&str]) -> anyhow::Result<ExitCode> {
    let (_, root, _) = load_package()?;
    let polish = std::env::var_os("SQUEEZE_POLISH")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| find_on_path("polish"));
    let Some(polish) = polish else {
        bail!(
            "`squeeze {}` needs polish, the Crush formatter and linter, and it is not installed\n  \
             install it from https://github.com/nixpt/polish and put `scripts/polish` on PATH as \
             `polish`, or set SQUEEZE_POLISH=/path/to/polish",
            args[0]
        );
    };
    let mut files = Vec::new();
    for dir in ["src", "tests"] {
        collect_crush(&root.join(dir), &mut files);
    }
    files.sort();
    if files.is_empty() {
        status("Finished", "no .crush files under src/ or tests/");
        return Ok(ExitCode::SUCCESS);
    }
    let st = std::process::Command::new(&polish)
        .args(args)
        .args(&files)
        .current_dir(&root)
        .stdin(std::process::Stdio::null())
        .status()
        .with_context(|| format!("running {}", polish.display()))?;
    Ok(exit_code(st.code().unwrap_or(1)))
}

fn collect_crush(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for e in rd.filter_map(Result::ok) {
        let p = e.path();
        if p.is_dir() {
            collect_crush(&p, out);
        } else if p.extension().is_some_and(|x| x == "crush") {
            out.push(p);
        }
    }
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|d| d.join(name))
        .find(|p| p.is_file())
}

/// `path` relative to `root` when it is inside it, for status lines.
fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}
