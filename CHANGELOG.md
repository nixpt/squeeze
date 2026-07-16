# Changelog

All notable changes to `squeeze` are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate adheres
to [Semantic Versioning](https://semver.org/) as documented in
`SQUEEZE-6-v1-cli-stability-contract.md`.

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
