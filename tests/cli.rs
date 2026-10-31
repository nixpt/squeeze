//! squeeze end to end: the real binary over temp directories, stdin closed,
//! no network (facet is the `tests/fixtures/mini-facet` test double; git
//! dependencies are local `file://` repos).

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

struct Env {
    tmp: tempfile::TempDir,
}

impl Env {
    fn new() -> Self {
        Env {
            tmp: tempfile::tempdir().unwrap(),
        }
    }

    fn root(&self) -> &Path {
        self.tmp.path()
    }

    fn cmd(&self, dir: &Path, args: &[&str]) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_squeeze"));
        c.args(args)
            .current_dir(dir)
            .env("CRUSH_PKG_CACHE", self.root().join("cache"))
            .env("NO_COLOR", "1")
            .env_remove("CRUSH_PKG_OFFLINE")
            .env_remove("CRUSH_PATH")
            .env_remove("SQUEEZE_ADMIT")
            .env_remove("SQUEEZE_ADMITTED")
            .env_remove("SQUEEZE_POLISH")
            .stdin(Stdio::null());
        c
    }

    fn run(&self, dir: &Path, args: &[&str]) -> Output {
        self.cmd(dir, args).output().expect("spawn squeeze")
    }

    /// `squeeze new NAME extra…` in the temp root; the package directory.
    fn new_pkg(&self, name: &str, extra: &[&str]) -> PathBuf {
        let mut args = vec!["new", name];
        args.extend(extra);
        let out = self.run(self.root(), &args);
        assert!(out.status.success(), "{}", text(&out));
        self.root().join(name)
    }
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn text(o: &Output) -> String {
    format!("{}{}", stdout(o), String::from_utf8_lossy(&o.stderr))
}

fn write(root: &Path, files: &[(&str, &str)]) {
    for (rel, src) in files {
        let p = root.join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, src).unwrap();
    }
}

fn mini_facet() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini-facet")
}

#[test]
fn new_build_run() {
    let env = Env::new();
    let app = env.new_pkg("hello", &[]);
    for f in [
        "capsule.toml",
        "src/main.crush",
        "tests/greeting.crush",
        ".gitignore",
    ] {
        assert!(app.join(f).is_file(), "missing {f}");
    }
    let build = env.run(&app, &["build"]);
    assert!(build.status.success(), "{}", text(&build));
    assert!(app.join("target/hello.cvm1").is_file() && app.join("target/hello.casm").is_file());
    assert!(
        text(&build).contains("Compiling hello v0.1.0"),
        "{}",
        text(&build)
    );

    let run = env.run(&app, &["run"]);
    assert!(run.status.success(), "{}", text(&run));
    // Status lines go to stderr; stdout is the program's alone.
    assert_eq!(stdout(&run), "hello from Crush\n", "{}", text(&run));

    let bare = env.run(&app, &[]);
    assert!(bare.status.success(), "{}", text(&bare));
    assert_eq!(stdout(&bare), "hello from Crush\n");

    let rel = env.run(&app, &["build", "--release"]);
    assert!(rel.status.success(), "{}", text(&rel));
    assert!(app.join("target/release/hello.cvm1").is_file());

    let clean = env.run(&app, &["clean"]);
    assert!(
        clean.status.success() && !app.join("target").exists(),
        "{}",
        text(&clean)
    );
}

#[test]
fn cli_scaffold_help_and_usage_error_codes() {
    let env = Env::new();
    let facet = mini_facet();
    let app = env.new_pkg(
        "mytool",
        &["--cli", "--facet-path", facet.to_str().unwrap()],
    );
    let main = std::fs::read_to_string(app.join("src/main.crush")).unwrap();
    assert!(main.contains("import facet { facet_run }"), "{main}");

    let help = env.run(&app, &["run", "--", "--help"]);
    assert_eq!(help.status.code(), Some(0), "{}", text(&help));
    assert!(stdout(&help).contains("Usage: mytool"), "{}", text(&help));

    let bogus = env.run(&app, &["run", "--", "--bogus"]);
    assert_eq!(bogus.status.code(), Some(2), "{}", text(&bogus));
    assert!(
        stdout(&bogus).contains("unrecognized option '--bogus'"),
        "{}",
        text(&bogus)
    );

    let words = env.run(&app, &["run", "--", "hi", "there"]);
    assert!(
        words.status.success() && stdout(&words).contains("hi there"),
        "{}",
        text(&words)
    );

    let test = env.run(&app, &["test"]);
    assert!(test.status.success(), "{}", text(&test));
    assert!(
        stdout(&test).contains("test result: ok. 1 passed; 0 failed"),
        "{}",
        text(&test)
    );
}

#[test]
fn multi_module_src_imports() {
    let env = Env::new();
    let app = env.new_pkg("mm", &[]);
    write(
        &app,
        &[
            (
                "src/main.crush",
                "import cli\n\nfn main() {\n    io.print(cli.banner(\"crush\"))\n}\n",
            ),
            (
                "src/cli.crush",
                "import util.strings as s\n\nfn banner(x) {\n    return s.shout(x)\n}\n",
            ),
            (
                "src/util/strings.crush",
                "import marks\n\nfn shout(x) {\n    return x + marks.bang()\n}\n",
            ),
            ("src/marks.crush", "fn bang() {\n    return \"!\"\n}\n"),
        ],
    );
    let run = env.run(&app, &["run"]);
    assert!(run.status.success(), "{}", text(&run));
    assert_eq!(stdout(&run), "crush!\n");
}

#[test]
fn test_reports_pass_and_fail_summary() {
    let env = Env::new();
    let app = env.new_pkg("t", &[]);
    write(
        &app,
        &[
            (
                "tests/b_fails.crush",
                "fn main() {\n    io.print(\"boom detail\")\n    sys.exit(4)\n}\n",
            ),
            (
                "tests/c_compile_error.crush",
                "import nowhere\nfn main() {\n}\n",
            ),
            (
                "tests/d_passes.crush",
                "fn main() {\n    io.print(\"quiet on success\")\n}\n",
            ),
        ],
    );
    let out = env.run(&app, &["test"]);
    assert_eq!(out.status.code(), Some(1), "{}", text(&out));
    let s = stdout(&out);
    assert!(s.contains("test tests/greeting.crush ... ok"), "{s}");
    assert!(s.contains("test tests/b_fails.crush ... FAILED"), "{s}");
    assert!(
        s.contains("test tests/c_compile_error.crush ... FAILED"),
        "{s}"
    );
    assert!(s.contains("test tests/d_passes.crush ... ok"), "{s}");
    assert!(
        s.contains("boom detail") && s.contains("exited with status 4"),
        "{s}"
    );
    assert!(s.contains("module not found"), "{s}");
    assert!(
        !s.contains("quiet on success"),
        "passing output is captured: {s}"
    );
    assert!(s.contains("test result: FAILED. 2 passed; 2 failed"), "{s}");

    let filtered = env.run(&app, &["test", "passes"]);
    assert!(filtered.status.success(), "{}", text(&filtered));
    assert!(
        stdout(&filtered).contains("test result: ok. 1 passed; 0 failed"),
        "{}",
        text(&filtered)
    );
}

#[test]
fn sys_exit_code_passes_through_run() {
    let env = Env::new();
    let app = env.new_pkg("ex", &[]);
    write(
        &app,
        &[(
            "src/main.crush",
            "fn main() {\n    io.print(\"before exit\")\n    sys.exit(3)\n}\n",
        )],
    );
    let out = env.run(&app, &["run"]);
    assert_eq!(out.status.code(), Some(3), "{}", text(&out));
    assert_eq!(stdout(&out), "before exit\n");
}

#[test]
fn add_path_dependency_keeps_comments_and_imports_by_name() {
    let env = Env::new();
    let _lib = env.new_pkg("str-lib", &["--lib"]);
    let app = env.new_pkg("app", &[]);
    let toml = app.join("capsule.toml");
    let mut original = std::fs::read_to_string(&toml).unwrap();
    original.insert_str(0, "# my app: keep this comment\n");
    std::fs::write(&toml, &original).unwrap();

    let add = env.run(&app, &["add", "str-lib", "--path", "../str-lib"]);
    assert!(add.status.success(), "{}", text(&add));
    let edited = std::fs::read_to_string(&toml).unwrap();
    assert!(
        edited.starts_with("# my app: keep this comment\n"),
        "{edited}"
    );
    assert!(
        edited.contains("[[dependencies]]\nname = \"str-lib\"\npath = \"../str-lib\""),
        "{edited}"
    );
    assert!(text(&add).contains("import str_lib"), "{}", text(&add));

    write(
        &app,
        &[(
            "src/main.crush",
            "import str_lib { greet }\n\nfn main() {\n    io.print(greet(\"deps\"))\n}\n",
        )],
    );
    let run = env.run(&app, &["run"]);
    assert!(run.status.success(), "{}", text(&run));
    assert_eq!(stdout(&run), "hello, deps\n");

    let itself = env.run(&app, &["add", "other", "--path", "."]);
    assert_eq!(itself.status.code(), Some(1), "{}", text(&itself));
    let own_name = env.run(&app, &["add", "app", "--path", "../str-lib"]);
    assert_eq!(own_name.status.code(), Some(1), "{}", text(&own_name));

    let missing = env.run(&app, &["add", "nope", "--path", "../does-not-exist"]);
    assert_eq!(missing.status.code(), Some(1), "{}", text(&missing));

    let rm = env.run(&app, &["remove", "str-lib"]);
    assert!(rm.status.success(), "{}", text(&rm));
    let after = std::fs::read_to_string(&toml).unwrap();
    assert!(
        !after.contains("str-lib") && after.starts_with("# my app"),
        "{after}"
    );
    let rm_again = env.run(&app, &["remove", "str-lib"]);
    assert_eq!(rm_again.status.code(), Some(1));
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "tag.gpgsign=false",
        ])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn add_git_dependency_pins_lock_and_imports() {
    let env = Env::new();
    let repo = env.root().join("gdlib");
    write(
        &repo,
        &[
            (
                "capsule.toml",
                "[capsule]\nname = \"gdlib\"\nversion = \"0.1.0\"\nentry = \"gd.crush\"\nlanguage = \"crush\"\n",
            ),
            (
                "gd.crush",
                "fn gd_greet() {\n    return \"hello from git\"\n}\n",
            ),
        ],
    );
    git(&repo, &["init", "-q", "-b", "main"]);
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "v1"]);
    git(&repo, &["tag", "-a", "v1", "-m", "v1"]);
    let url = format!("file://{}", repo.display());

    let app = env.new_pkg("app", &[]);
    let bad = env.run(&app, &["add", "gdlib", "--git", &url, "--tag", "v9"]);
    assert_eq!(bad.status.code(), Some(1), "{}", text(&bad));
    assert!(
        !std::fs::read_to_string(app.join("capsule.toml"))
            .unwrap()
            .contains("gdlib"),
        "bad tag left a row"
    );

    let add = env.run(&app, &["add", "gdlib", "--git", &url, "--tag", "v1"]);
    assert!(add.status.success(), "{}", text(&add));
    let lock = std::fs::read_to_string(app.join("capsule.lock")).unwrap();
    assert!(
        lock.contains("name = \"gdlib\"") && lock.contains("ref = \"tag:v1\""),
        "{lock}"
    );

    write(
        &app,
        &[(
            "src/main.crush",
            "import gdlib { gd_greet }\n\nfn main() {\n    io.print(gd_greet())\n}\n",
        )],
    );
    // Offline works now: add fetched it into the cache and pinned it.
    let run = env.run(&app, &["--offline", "run"]);
    assert!(run.status.success(), "{}", text(&run));
    assert_eq!(stdout(&run), "hello from git\n");

    let rm = env.run(&app, &["remove", "gdlib"]);
    assert!(rm.status.success(), "{}", text(&rm));
    assert!(
        !app.join("capsule.lock").exists(),
        "lock dropped with the last git dependency"
    );

    // add needs a source, and one git reference
    let none = env.run(&app, &["add", "x"]);
    assert_eq!(none.status.code(), Some(2), "{}", text(&none));
    let two = env.run(
        &app,
        &["add", "x", "--git", &url, "--tag", "v1", "--branch", "main"],
    );
    assert_eq!(two.status.code(), Some(2), "{}", text(&two));
}

#[cfg(unix)]
fn script(path: &Path, body: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(path, format!("#!/bin/sh\n{body}\n")).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(unix)]
#[test]
fn admit_hook_wraps_build_work_only() {
    let env = Env::new();
    let app = env.new_pkg("adm", &[]);
    let log = env.root().join("admit.log");
    let wrapper = env.root().join("echo-wrapper");
    // Records its arguments, then runs what follows `--`.
    script(
        &wrapper,
        &format!(
            "echo \"$@\" >> {}\n[ \"$1\" = \"--\" ] && shift\nexec \"$@\"",
            log.display()
        ),
    );
    let prefix = format!("{} --", wrapper.display());

    let run = env
        .cmd(&app, &["run"])
        .env("SQUEEZE_ADMIT", &prefix)
        .output()
        .unwrap();
    assert!(run.status.success(), "{}", text(&run));
    assert_eq!(stdout(&run), "hello from Crush\n");
    let exit3 = {
        write(
            &app,
            &[("src/main.crush", "fn main() {\n    sys.exit(3)\n}\n")],
        );
        env.cmd(&app, &["run"])
            .env("SQUEEZE_ADMIT", &prefix)
            .output()
            .unwrap()
    };
    assert_eq!(
        exit3.status.code(),
        Some(3),
        "wrapper passes the status through: {}",
        text(&exit3)
    );
    let clean = env
        .cmd(&app, &["clean"])
        .env("SQUEEZE_ADMIT", &prefix)
        .output()
        .unwrap();
    assert!(clean.status.success());

    let logged = std::fs::read_to_string(&log).unwrap();
    let lines: Vec<&str> = logged.lines().collect();
    assert_eq!(lines.len(), 2, "run twice, clean not wrapped: {logged}");
    assert!(
        lines[0].starts_with("-- ") && lines[0].ends_with("squeeze run"),
        "{logged}"
    );

    // A wrapper that refuses admission stops the build.
    let deny = env.root().join("deny");
    script(&deny, "echo denied >&2\nexit 75");
    let denied = env
        .cmd(&app, &["build"])
        .env("SQUEEZE_ADMIT", deny.to_str().unwrap())
        .output()
        .unwrap();
    assert_eq!(denied.status.code(), Some(75), "{}", text(&denied));
}

#[cfg(unix)]
#[test]
fn fmt_and_lint_delegate_to_polish() {
    let env = Env::new();
    let app = env.new_pkg("fm", &[]);
    let missing = env
        .cmd(&app, &["fmt"])
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(1), "{}", text(&missing));
    assert!(
        text(&missing).contains("needs polish"),
        "{}",
        text(&missing)
    );

    let bin = env.root().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let log = env.root().join("polish.log");
    script(
        &bin.join("polish"),
        &format!(
            "echo \"$@\" >> {}\n[ \"$1\" = lint ] && exit 1\nexit 0",
            log.display()
        ),
    );
    let path = format!("{}:/usr/bin:/bin", bin.display());
    let fmt = env
        .cmd(&app, &["fmt", "--check"])
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(fmt.status.success(), "{}", text(&fmt));
    let lint = env
        .cmd(&app, &["lint"])
        .env("PATH", &path)
        .output()
        .unwrap();
    assert_eq!(
        lint.status.code(),
        Some(1),
        "polish's findings status passes through"
    );
    let logged = std::fs::read_to_string(&log).unwrap();
    let lines: Vec<&str> = logged.lines().collect();
    assert!(
        lines[0].starts_with("fmt --check ")
            && lines[0].contains("src/main.crush")
            && lines[0].contains("tests/greeting.crush"),
        "{logged}"
    );
    assert!(lines[1].starts_with("lint "), "{logged}");

    // $SQUEEZE_POLISH wins over PATH.
    let other = env.root().join("other-polish");
    script(&other, "exit 0");
    let via_env = env
        .cmd(&app, &["lint"])
        .env("PATH", &path)
        .env("SQUEEZE_POLISH", &other)
        .output()
        .unwrap();
    assert!(via_env.status.success(), "{}", text(&via_env));
}

#[test]
fn usage_errors_exit_2_and_help_exits_0() {
    let env = Env::new();
    for cmd in [
        "new", "build", "run", "check", "test", "clean", "add", "remove", "fmt", "lint",
    ] {
        let help = env.run(env.root(), &[cmd, "--help"]);
        assert_eq!(help.status.code(), Some(0), "{cmd} --help: {}", text(&help));
    }
    assert_eq!(env.run(env.root(), &["frobnicate"]).status.code(), Some(2));
    assert_eq!(env.run(env.root(), &["new"]).status.code(), Some(2));
    assert_eq!(
        env.run(env.root(), &["new", "x", "--cli", "--lib"])
            .status
            .code(),
        Some(2)
    );
    let nowhere = env.run(env.root(), &["build"]);
    assert_eq!(nowhere.status.code(), Some(1), "{}", text(&nowhere));
    assert!(
        text(&nowhere).contains("could not find capsule.toml"),
        "{}",
        text(&nowhere)
    );
}

#[test]
fn new_lib_builds_and_tests() {
    let env = Env::new();
    let lib = env.new_pkg("my-lib", &["--lib"]);
    assert!(lib.join("src/lib.crush").is_file());
    let test = env.run(&lib, &["test"]);
    assert!(test.status.success(), "{}", text(&test));
    assert!(
        stdout(&test).contains("test result: ok. 1 passed; 0 failed"),
        "{}",
        text(&test)
    );
    let exists = env.run(env.root(), &["new", "my-lib"]);
    assert_eq!(exists.status.code(), Some(1), "{}", text(&exists));
}
