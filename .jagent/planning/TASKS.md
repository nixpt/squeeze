# TASKS — squeeze

Sub-tasks of the roadmap. Milestones (ROADMAP.md) are the unit of strategy; tasks are the unit of pull requests.

## P0 — Skeleton

- [x] `squeeze new <name>` scaffolds via `crush_pkg::manifest::scaffold_package`
- [x] `squeeze check` / `squeeze build` / `squeeze run` — thin wrappers over `PackageBuilder`
- [x] Default (no subcommand): check → build → write `target/` → run, one status line per stage
- [x] Non-`crush` `language` value errors explicitly (`require_crush_language`), not silently mis-handled
- [x] Verified end-to-end: real `squeeze new demo` → `target/demo.cvm` + `.casm.json` → `hello from Crush`

## P1 — Test infrastructure (M1)

- [ ] **SQUEEZE-2** — comprehensive test suite + CI (see ticket)
  - [ ] `tempfile` dev-dep; harness follows `crush-pkg`'s tempdir + `scaffold_package` pattern
  - [ ] Cover: `new`, `check`, `build`, `run`, default flow, two documented error paths
  - [ ] Status-line format tests (literal `checking <name> v<version>` / `building` / `running` / `done: N function(s), N byte(s)` etc.)
  - [ ] Exit-code tests (success=0, runtime=1, parse=2, wrong-language=3, missing-manifest=4)
- [ ] **SQUEEZE-3** — CI pipeline + crates.io publishing (lands alongside SQUEEZE-2 as M5)
  - [ ] `.github/workflows/ci.yml`: `cargo test` on stable + MSRV, deny advisories
  - [ ] `rust-toolchain.toml` pinning MSRV
  - [ ] Tag-driven release workflow; first publication tagged `v0.1.0`

## P2 — Script/Native + ecosystem integration (M2/M3/M4)

- [ ] **SQUEEZE-1** — Subcommand dispatch rewrite (full cutover)
  - [ ] `cmd_run` + `cmd_default`'s run leg go through `crush_pkg::runners::handle_run()` for every capsule type
  - [ ] `cmd_build` / `cmd_check` stay on `PackageBuilder` for crush; error out explicitly (right-shape "doesn't apply") for non-crush
  - [ ] `require_crush_language` either goes away or narrows to `cmd_build`/`cmd_check` only
  - [ ] Default flow on non-crush: skip check/build, go straight to run
- [ ] **SQUEEZE-4** — Wasm/crush-web verification (M3, expected doc-only)
  - [ ] Verify `crush-web`'s `run_blob` accepts `squeeze build`'s `.cvm` output
  - [ ] Minimal example; README "Web target" section
- [ ] **SQUEEZE-5** — Distribution wrappers (M4)
  - [ ] `squeeze pack` / `squeeze sign` / `squeeze verify` wrapping `crush_pkg::ops::{pack, sign, verify}`

## P3 — CLI contract (M6)

- [ ] **SQUEEZE-6** — v1.0 CLI stability contract (M6)
  - [ ] Factor status-line literals into `src/messages.rs`
  - [ ] Type exit codes (`SqueezeExit` enum)
  - [ ] `MANUAL.md` listing every subcommand, flag, exit code, status-line literal
  - [ ] `CHANGELOG.md` from v0.x → v1.0; post-v1.0 breaking-change gate
