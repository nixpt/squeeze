# SQUEEZE-2 — Comprehensive test infrastructure (M1)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-2 |
| **Priority** | P1 |
| **Status** | Backlog |
| **Phase** | M1 |
| **Assignee** | unassigned |
| **Dependencies** | none (sets the regression net SQUEEZE-1 needs) |
| **Estimated effort** | M (~1 PR, ~6–10 hours of writing) |

## Problem

There are zero tests under `squeeze/`. Without tests, changing the load path (SQUEEZE-1 / M2 — `cmd_run` cuts over to `crush_pkg::runners::handle_run()`) is unsafe: a regression in dispatch silently swallows non-crush capsules, and we lose the documented CLI contract (status-line strings, exit codes, error format) the moment a contributor rewrites a `println!` to "tidy it up". M1's scope is **comprehensive** — the contract is what locks in M6's stability promise, so it's cheaper to capture now than to retrofit once v1.0 ships.

## Success criteria

- [ ] Dev-dependency on `tempfile` (mirroring `crush-pkg`'s own test setup) added to `Cargo.toml`
- [ ] Test harness follows `crush-pkg`'s own pattern: `tempfile::tempdir()` + `crush_pkg::manifest::scaffold_package(&dir, name)` + asserts on `target/` contents (and stdout/stderr/exit code for command-level tests)
- [ ] Command-level tests (asserting stdout/stderr/exit code, not just filesystem state):
  - [ ] `squeeze new <name>` creates a scaffolded `capsule.toml` + entry source in the named dir
  - [ ] `squeeze check` on a crush capsule exits 0 and prints the documented status line
  - [ ] `squeeze build` on a crush capsule exits 0, writes `target/<name>.cvm` + `target/<name>.casm.json`
  - [ ] `squeeze run` on a crush capsule exits 0, prints the entry source's output
  - [ ] `squeeze` (default flow) on a crush capsule exits 0, prints the documented three-stage status lines, runs the entry source
  - [ ] `squeeze build` / `squeeze check` / `squeeze run` / `squeeze` on a missing `capsule.toml` exits with the documented "no manifest" error and exit code
  - [ ] `squeeze build` / `squeeze check` on a non-crush capsule exits with the documented "doesn't apply" error (post M2's rewrite; test for the v0.1 generic refusal can stay until M2 lands, then be updated)
- [ ] Status-line format tests: literal status-line strings each get a focused test asserting exact text. Capture both the stand-alone subcommand output AND the default-flow’s three-stage output, since the two diverge in indentation and `done:`-prefixing:
  - [ ] `checking <name> v<version>` (both stand-alone `squeeze check` and default-flow stage 1)
  - [ ] `building <name> v<version>` (both stand-alone `squeeze build` and default-flow stage 2)
  - [ ] `running <name>` (both stand-alone `squeeze run` and default-flow stage 3)
  - [ ] `done: N function(s), N byte(s)` (stand-alone `squeeze build`’s tail line only)
  - [ ] `  N function(s), N byte(s)` (default-flow stage 2’s tail line — TWO-space indent, NO `done:` prefix; literal-string equality)
- [ ] Exit-code contract (mirrors SQUEEZE-6's `SqueezeExit` enum, named verbatim so MANUAL.md doesn't drift): Ok=0, Runtime=1, ManifestParse=2 (manifest file exists but won't parse — from `Manifest::from_file` failing), WrongLanguage=3, MissingManifest=4 (no capsule.toml/Capsule.toml/crush.toml/Crush.toml found in cwd — from `manifest_path(&cwd)` returning None).
- [ ] All tests run under `cargo test` with no extra setup

## Technical approach

1. Add `tempfile = "3"` (or whatever version `crush-pkg` uses in its own tests; reuse, don't guess) to `[dev-dependencies]`. No other new dependencies.
2. Place tests in `src/main.rs` under `#[cfg(test)] mod tests`, using `assert_cmd` if `crush-pkg` uses it, otherwise hand-rolled `std::process::Command::new(env!("CARGO_BIN_EXE_squeeze"))`.
3. Each test creates a tempdir, scaffolds an in-memory capsule (small library of fixture TOML strings: crush-source, "python" with hello-world script), invokes `squeeze` as a subprocess, asserts exit code + stdout + stderr + filesystem side-effects.
4. Status-line format tests are literal-string equality, not substring matches. This is M6's contract in test form.
5. `cargo test` locally before merging. CI runs it on every PR once SQUEEZE-3 lands.

## Files to modify

- `Cargo.toml` — add `[dev-dependencies]` (`tempfile`, optionally `assert_cmd`)
- `src/main.rs` — add `#[cfg(test)] mod tests` block
- Optional: `tests/cli.rs` integration-test file if the unit-style `#[cfg(test)]` approach feels cramped for subprocess tests

## Non-goals

- Property-based / fuzz testing — out of scope for v0.x
- Coverage thresholds (`tarpaulin`) — informal measurement only
- Performance benchmarks — `squeeze` is fast enough by construction
