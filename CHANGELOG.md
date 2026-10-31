# Changelog

All notable changes to `squeeze` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate adheres
to [Semantic Versioning](https://semver.org/) as documented in
`SQUEEZE-6-v1-cli-stability-contract.md`.

## [Unreleased]

## [0.2.0] - 2026-10-31

squeeze is Crush's cargo again (SQUEEZE-8): the front door over crush-pkg,
reversing crush-ast decision C-4 (CRUSH-167). Built on crush-pkg and
crush-lang-sdk 0.10.0 (CRUSH-367 imports through the package).

### Release

- CI (`ci.yml`: fmt, clippy, tests, `cargo package`, MSRV check) and a
  tag-driven `release.yml`: crates.io via Trusted Publishing plus a GitHub
  release with an x86_64 Linux binary and `SHA256SUMS` (SQUEEZE-3).
- `rust-version` is 1.95, the floor crush-pkg 0.10 sets (was 1.85, which
  could not build).

### Added

- The standard layout (`src/main.crush`, `tests/*.crush`, `capsule.lock`,
  `target/`), with imports through the package: `src/` is a search root,
  and each dependency (and the package itself) is importable by name, so
  `import facet { facet_run }` works from a git dependency.
- `squeeze new NAME --cli` (facet @ v0.2.0) and `--lib`.
- `squeeze run -- ARGS…` forwards arguments (`sys.args()`). `--fs`, `--env`
  and `--time` grant capabilities. The program's `sys.exit(n)` becomes
  squeeze's exit status.
- `squeeze build --release` writes to `target/release/`. crush-frontend has
  no optimisation levels, so the code is the same as a debug build.
- `squeeze test [FILTER]`: runs each `tests/*.crush` in its own process,
  shows output only for failures, and ends with `test result:`.
- `squeeze clean`, `squeeze add` (`--git` + `--tag`/`--rev`/`--branch`, or
  `--path`; updates `capsule.lock`, keeps the comments in `capsule.toml`)
  and `squeeze remove`.
- `squeeze fmt [--check]` / `squeeze lint` hand off to polish
  (`$SQUEEZE_POLISH` or `polish` on PATH).
- `SQUEEZE_ADMIT`: a command prefix that build, run, check and test re-run
  themselves under (fleet build admission, e.g. buildsched/bob).
- `--offline` on every command.
- `tests/cli.rs`: 11 end-to-end tests over the real binary (SQUEEZE-2, part).

### Changed

- **Status lines and exit codes (breaking).** The old status lines
  (`checking <name> v<version>`, `building …`, `running …`, `done: N function(s)…`)
  are replaced by cargo-style lines on **stderr**
  (`   Compiling NAME vX (DIR)`, `    Finished …`, `     Running …`), so stdout
  carries only the program's output. Exit codes are 0 ok, 1 failure,
  2 usage error, with the program's own status passed through on `run`.
  Neither had been ratified (SQUEEZE-6).
- The bare `squeeze` builds and runs, as before, but no longer runs a
  separate check pass first (the build already compiles the whole package).
- `build` writes `target/NAME.cvm1` + `target/NAME.casm` (crush-pkg's
  formats since crush-ast #365), not `.cvm` + `.casm.json`.
- Depends on crates.io `crush-pkg` and `crush-lang-sdk` 0.9 (plus
  `toml_edit`).

- Published to crates.io as **`crush-squeeze`**: `squeeze` there is an
  unrelated compression library. The binary is still `squeeze`
  (`cargo install crush-squeeze`).

- Port to crush-ast 0.8: `crush-pkg` now comes from crates.io (`0.8.0`)
  instead of a sibling path-dep on 0.3.0. Closures now compile and run.

### Removed

- Dead `crush-vm` dependency (SQUEEZE-7); nothing in `src/` used it.

## [0.1.0] - 2026-07-16

The first crates.io-bound smoke publish of `squeeze`. Ships exactly what
already lives on `master` (commit `63cb23d`); no Rust source changes in this
release. The point of publishing now is to unblock downstream consumer peers
(crush-web, crush-notebook) that want `cargo install squeeze` before
[M1 / SQUEEZE-2](tickets/SQUEEZE-2-comprehensive-test-coverage.md)'s test
suite lands.

### Added

- CLI skeleton: `squeeze new`, `squeeze check`, `squeeze build`, `squeeze run`,
  and the intuitive default flow (`squeeze` with no subcommand = check + build
  + write `target/` + run).- One-status-line-per-stage UX (cargo-style). The exact literal strings (per `src/main.rs`):

  - `checking <name> v<version>` — from both `squeeze check` and the default flow's stage 1
  - `building <name> v<version>` — from both `squeeze build` and the default flow's stage 2
  - `running <name>` — from both `squeeze run` and the default flow's stage 3
  - `done: N function(s), N byte(s)` — literal, from `squeeze build` ONLY
  - `  N function(s), N byte(s)` — **two-space indent, NO `done:` prefix**, from the default flow's stage 2 build-output line

  These two `done:` / indented variants differ. Captured here so [M6 / SQUEEZE-6](tickets/SQUEEZE-6-v1-cli-stability-contract.md) can ratify BOTH as part of the v1.0 stability contract without retrofitting.
- Verified end-to-end: `squeeze new demo` → `target/demo.cvm` + `demo.casm.json`
  → `hello from Crush` output, both via the default flow and via `squeeze run`
  with no prior build.

### Constraints (still on master)

- **`language = "crush"` capsules only.** Script (bun/node/deno/python) and
  Native capsules hit an explicit refusal — see the module doc in `src/main.rs`.
  Full cutover to `crush_pkg::runners::handle_run()` is tracked in
  [SQUEEZE-1](tickets/SQUEEZE-1-script-native-capsules.md) (M2).
- **No tests.** Test infrastructure is tracked in
  [SQUEEZE-2](tickets/SQUEEZE-2-comprehensive-test-coverage.md) (M1).
- **`squeeze 0.1.0` cannot be published to crates.io today.** The crate
  depends on `crush-pkg v0.3.0` and `crush-vm v0.3.0`, neither of which is on
  crates.io yet. The dependency structure on this `release/v0.1.0` branch is
  publish-ready — see `RELEASE.md` for the publish sequence once upstream
  lands.
