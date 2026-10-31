# SQUEEZE-8 — squeeze is Crush's cargo

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-8 |
| **Priority** | P1 |
| **Status** | Done (PR open) |
| **Assignee** | satya |
| **Dependencies** | crush-ast CRUSH-367 (crush-pkg import mode + scaffolds + builder hooks) |

## Problem

The captain wants one command to create, build, run and test Crush programs
("so we can compile crush programs through it"). crush-ast decision C-4
(CRUSH-167, s474) had folded squeeze's build-then-run into crush-pkg and
slated this repo for archiving. SQUEEZE-8 revives squeeze as the user-facing
front door, with crush-pkg as the engine. squeeze never reimplements
crush-pkg's pipeline (dejavue decision, 2026-10-31).

## Success criteria

- [x] Standard layout (captain approved: "what squeeze new generates is the standard"):
      `capsule.toml`, `capsule.lock`, `src/main.crush`, `tests/*.crush`, `target/`.
- [x] Imports through the package: `src/` + each dependency by name (crush-ast CRUSH-367).
- [x] `new [--cli|--lib]`, `build [--release]`, `run [-- ARGS]`, `check`, `test`,
      `clean`, `add`, `remove`, `fmt [--check]`, `lint`, bare `squeeze` = build + run;
      each takes `--help`.
- [x] Exit codes 0 / 1 / 2; `run` passes `sys.exit(n)` through.
- [x] `SQUEEZE_ADMIT` admission hook (bob), with no reimplementation of bob's preflight.
- [x] Integration tests (tests/cli.rs, 11); live proof against GitHub facet v0.2.0.
- [x] SQUEEZE-7 folded in (crush-vm dep stays gone; nothing needs it).

## Resolution

`src/main.rs` (commands), `src/edit.rs` (toml_edit for add/remove), `src/admit.rs`
(re-exec under `$SQUEEZE_ADMIT` with `SQUEEZE_ADMITTED=1`), `src/status.rs` (cargo-style
stderr lines). The crush-pkg API squeeze needed was added upstream in crush-ast
CRUSH-367 rather than here. Release needs the next crush-ast release on crates.io
(RELEASE.md).

Advances: SQUEEZE-1 (arg forwarding: done), SQUEEZE-2 (CLI integration tests; no CI or
script-capsule matrix yet), SQUEEZE-6 (status lines and exit codes redefined; not
ratified). Not touched: SQUEEZE-3 (CI), SQUEEZE-4 (crush-web), SQUEEZE-5 (pack/sign/verify).

Not done: no `git init` in `squeeze new`; `--release` only picks an output dir (the
frontend has no optimisation levels); transitive dependencies are not followed
(crush-pkg limitation); the `SQUEEZE_ADMIT` prefix is split on whitespace only.
