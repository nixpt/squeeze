# squeeze

**Crush's cargo**: the one command to create, build, run and test
[Crush](https://github.com/nixpt/crush-ast) programs.

```bash
squeeze new hello --cli        # a command-line program, facet as a git dependency
cd hello
squeeze run -- --help          # fetch facet, build, run
squeeze test                   # run tests/*.crush
squeeze build --release        # target/release/hello.cvm1 + .casm
```

squeeze is the front door; `crush-pkg` (crush-ast) is the engine underneath.
Manifest parsing, dependency and git resolution, `capsule.lock`, imports,
compilation, the bytecode writers and the VM runners are all `crush-pkg`;
squeeze never reimplements them. It adds the cargo-shaped command surface on
top.

## The standard layout

What `squeeze new` generates (and what every command expects):

```text
myapp/
  capsule.toml   name, version, entry = "src/main.crush", [[dependencies]], [capabilities]
  capsule.lock   pinned git revisions (written by the first build or `squeeze add`; commit it)
  src/main.crush fn main(); `import cli` → src/cli.crush, `import util.strings` → src/util/strings.crush
  tests/*.crush  one program per file, run by `squeeze test`; passes when it exits 0
  target/        build output (in .gitignore)
```

```toml
[capsule]
name = "myapp"
version = "0.1.0"
entry = "src/main.crush"
language = "crush"

[capabilities]
required = ["io.print", "sys.args", "sys.exit"]

[[dependencies]]
name = "facet"
git = "https://github.com/nixpt/facet"
tag = "v0.2.0"                 # or rev = "<sha>" / branch = "main" / path = "../facet"
```

### Imports

Code reaches other files and dependencies with `import`; nothing is pasted
together.

```crush
import facet { facet_run }     // a dependency, by its name: loads its entry file
import cli                     // src/cli.crush
import util.strings as s       // src/util/strings.crush

fn main() {
    facet_run(cli.spec(), { main: |args| => cli.run(args) })
}
```

- `src/` is a search root, so any file under it imports by its path from `src/`.
- Each dependency is importable by its `name` (`-` becomes `_`: `my-lib` →
  `import my_lib`). `import facet.env` loads `env.crush` next to facet's entry.
- The package itself is importable by its own name. That is how
  `tests/*.crush` reach `src/`: `import myapp { helper }`.

The language rules are in crush-ast's `crush-frontend/src/import_system.rs`;
the package rules in crush-pkg's `MANIFEST.md`. A package whose entry has no
`import` still builds the old way (dependency sources concatenated), so
older packages keep working.

## Commands

| Command | What it does |
|---|---|
| `squeeze new NAME [--cli \| --lib] [--dir DIR]` | Create a package in the standard layout. `--cli`: `import facet { facet_run }` with facet @ v0.2.0. `--lib`: `src/lib.crush`, no `main`. |
| `squeeze build [--release]` | Compile to `target/NAME.cvm1` + `target/NAME.casm`. `--release` writes to `target/release/` instead. crush-frontend has no optimisation levels (its optimizer always runs), so the code is the same. |
| `squeeze run [--release] [--fs] [--env] [--time] [-- ARGS…]` | Build, then run. `ARGS` are the program's `sys.args()`. `--fs`/`--env`/`--time` grant those capabilities. |
| `squeeze` | Same as `squeeze run`. |
| `squeeze check` | Compile without writing `target/`, then compare the capabilities the program uses with `[capabilities]`. |
| `squeeze test [FILTER]` | Compile and run each `tests/*.crush` in its own process, with `src/` and the dependencies importable. Output is shown only for failures. Ends with `test result: ok. N passed; M failed`. |
| `squeeze clean` | Remove `target/`. |
| `squeeze add NAME --git URL (--tag T \| --rev R \| --branch B)` | Add or replace a git dependency in `capsule.toml` and pin it in `capsule.lock`. A bad URL or tag changes nothing. |
| `squeeze add NAME --path DIR` | Add a path dependency (written relative to the package). |
| `squeeze remove NAME` | Remove a dependency from `capsule.toml` and `capsule.lock`. |
| `squeeze fmt [--check]` / `squeeze lint` | Run [polish](https://github.com/nixpt/polish) over `src/` and `tests/`. Uses `$SQUEEZE_POLISH` if set, else `polish` on PATH. Says how to install it if neither exists. |

Every command takes `--help`. `--offline` (or `CRUSH_PKG_OFFLINE=1`) never
touches the network: git dependencies come from `capsule.lock` and the
cache only. `add` and `remove` edit `capsule.toml` with `toml_edit`, so
comments and formatting elsewhere in the file are kept.

**Output.** Status lines (`Compiling`, `Finished`, `Running`, …) go to
stderr in cargo's style, so stdout carries only the program's output.

**Exit codes.** 0 means success, 1 a failure (compile error, failed test,
bad dependency), 2 a usage error. `run` and a bare `squeeze` exit with the
program's own status, so `sys.exit(3)` gives 3. `fmt` and `lint` pass
polish's status through (1 means findings).

**Script and native capsules.** `squeeze run` on a capsule whose `language`
is python/node/bun/deno/sona/native hands it to crush-pkg's runner. `build`,
`check` and `test` apply only to Crush source.

## Build admission (`SQUEEZE_ADMIT`)

On a shared box, builds can be routed through an admission scheduler such as
the fleet's buildsched (bob). squeeze doesn't decide admission itself. When
`SQUEEZE_ADMIT` holds a command prefix, squeeze re-runs itself under it for
`build`, `run`, `check`, `test` and bare `squeeze`:

```bash
SQUEEZE_ADMIT="bob admit --" squeeze build --release
#   runs: bob admit -- /path/to/squeeze build --release   (with SQUEEZE_ADMITTED=1)
```

The inner squeeze sees `SQUEEZE_ADMITTED=1` and does the work. The outer one
exits with the wrapper's status, so a wrapper that refuses admission stops
the build. The prefix is split on whitespace (no shell quoting). `new`,
`add`, `remove`, `clean`, `fmt` and `lint` are never wrapped.

## Install

```bash
cargo install crush-squeeze   # installs the `squeeze` binary
```

The crate is `crush-squeeze` because `squeeze` on crates.io is an unrelated
compression library. The command is still `squeeze`. Running a built
`target/NAME.cvm1` without squeeze uses `crush-run run target/NAME.cvm1 -- ARGS`
(from `cargo install crush-lang-sdk`).

## Developing against local crush-ast

squeeze depends on crates.io `crush-pkg`. This version needs crush-pkg API
from crush-ast CRUSH-367 that isn't released yet (see `RELEASE.md`). To
build against a crush-ast checkout, create a gitignored
`.cargo/config.toml`:

```toml
[patch.crates-io]
crush-pkg = { path = "/path/to/crush-ast/crates/crush-pkg" }
crush-lang-sdk = { path = "/path/to/crush-ast/crates/crush-lang-sdk" }
```

Patch both crates. If only `crush-pkg` is patched, two copies of
`crush-lang-sdk` get built, and their types don't match. Don't commit the
`Cargo.lock` this produces, because it records path sources.

```bash
cargo test          # tests/cli.rs drives the real binary over temp dirs (no network)
cargo clippy --all-targets -- -D warnings
```

## Provenance

Named after Crush's fruit-crushing metaphor (crush the grapes, squeeze the
juice out: that's the build). In s474 (crush-ast CRUSH-167, decision C-4),
squeeze's build-then-run flow was folded into `crush-pkg`, and squeeze was
due to be archived. In SQUEEZE-8 it came back as the front door, with
crush-pkg as the engine. See `.dejavue/decisions.md`.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE) at your option.
