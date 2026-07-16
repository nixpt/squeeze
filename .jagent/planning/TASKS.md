# TASKS — squeeze

Sub-tasks of the roadmap. Milestones (ROADMAP.md) are the unit of strategy; tasks are the unit of pull requests.

## P0 — Skeleton

- [x] `squeeze new <name>` scaffolds via `crush_pkg::manifest::scaffold_package`
- [x] `squeeze check` / `squeeze build` / `squeeze run` — thin wrappers over `PackageBuilder`
- [x] Default (no subcommand): check → build → write `target/` → run, one status line per stage
- [x] Non-`crush` `language` value: `run` dispatches via `crush_pkg::runners::get_runner_for_payload` (every capsule type); `check`/`build` refuse explicitly via `require_crush_buildable` (not silent mis-handling, not the v0.0.1 generic message)
- [x] Verified end-to-end: real `squeeze new demo` → `target/demo.cvm` + `.casm.json` → `hello from Crush`

## P1 — Test infrastructure (M1)

- [ ] **SQUEEZE-2** — comprehensive test suite + CI (see ticket)
  - [ ] `tempfile` dev-dep; harness follows `crush-pkg`'s tempdir + `scaffold_package` pattern
  - [ ] Cover: `new`, `check`, `build`, `run`, default flow, two documented error paths
  - [ ] Status-line format tests (literal `checking <name> v<version>` / `building` / `running` / `done: N function(s), N byte(s)` etc.)
  - [ ] Exit-code tests (success=0, runtime=1, parse=2, wrong-language=3, missing-manifest=4)
  - [ ] Dispatch-correctness regression net across capsule types (crush / native / bun / node / deno / python / sona) — sharpened by M2 cutover
- [ ] **SQUEEZE-3** — CI pipeline + crates.io publishing (lands alongside SQUEEZE-2 as M5)
  - [ ] `.github/workflows/ci.yml`: `cargo test` on stable + MSRV, deny advisories
  - [ ] `rust-toolchain.toml` pinning MSRV
  - [ ] Tag-driven release workflow; first publication tagged `v0.1.0`

## P2 — Script/Native + ecosystem integration (M2/M3/M4)

- [x] **SQUEEZE-1** — Subcommand dispatch rewrite (full cutover) — *code-complete on `feature/m2-script-native-cutover` @ `a41b546`; reviewer "no issues" + 1 micro-nit; awaiting master merge*
  - [x] `cmd_run` + `cmd_default`'s run leg go through `crush_pkg::runners::get_runner_for_payload` for every capsule type
  - [x] `cmd_build` / `cmd_check` stay on `PackageBuilder` for crush; error out explicitly (right-shape "doesn't apply") for non-crush
  - [x] `require_crush_language` removed; replaced with `require_crush_buildable` (per-subcommand guard in `cmd_build` / `cmd_check` only)
  - [x] Default flow on non-crush: skip check/build, go straight to run
  - [ ] **M2-deferred follow-up (not blocking):** `cmd_run` args forwarding to `runner.run(&manifest, &entry, args)`. Today `args` is the empty slice; `squeeze run <path> -- foo bar` does not surface `foo`/`bar` to the run.
  - [ ] **M2-deferred follow-up (not blocking):** `(halted)` status line for `ExecutionResult::Process(Child)`. Today the arm just reports the dispatch without a follow-up `wait()` status; the surface is trivially added once `Cargo.lock` / lifecycle is in place.
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
