# Planning state — squeeze

**Updated:** 2026-07-16
**Milestone focus:** M1 — test coverage (comprehensive). M2 code-complete on a separate branch.
**Branch:** M0 on `master`; planning refresh + M2 doc-follow-up on `planning/m1-m6-roadmap-refresh` (worktree at `../squeeze--planning-m1-m6`, unmerged); M2 code change on `feature/m2-script-native-cutover` (worktree at `../squeeze--m2-script-native-cutover`, unmerged).

## Delivery snapshot

| Track | Status | Notes |
|-------|--------|-------|
| CLI skeleton (M0) | shipped | `new` / `check` / `build` / `run` + default flow; verified end-to-end. |
| PackageBuilder integration (M0) | shipped | Peer path-dep; no reimplementation of compile/run. |
| Script/Native cutover (M2) | code-complete | `feature/m2-script-native-cutover` @ `a41b546`; reviewer "no issues" + 1 micro-nit. |
| Tests (M1) | missing | SQUEEZE-2 backlog; M1 still lands (regression net covers dispatch + roadmap). |
| CI (M5) | missing | SQUEEZE-3 backlog; lands with SQUEEZE-2. |
| Wasm/crush-web (M3) | missing | SQUEEZE-4; expected mostly doc/example work post-publish. |
| Distribution (M4) | missing | SQUEEZE-5; wraps `crush-pkg`'s `pack`/`sign`/`verify`. |
| crates.io publishing (M5) | repo + branch live | `nixpt/squeeze` is public; branch `release/v0.1.0` @ `d42710a` carries registry-pure prep (unmerged). |
| v1.0 CLI stability contract (M6) | missing | SQUEEZE-6. |
| Directory move | shipped | `projects/crush-workspace/` relocation completed 2026-07-14. |

## Active work

M2 (SQUEEZE-1) is **code-complete on `feature/m2-script-native-cutover`** @ `a41b546`. Single Rust file touched (`src/main.rs` 119 → 227 lines). `cmd_run` and the run leg of `cmd_default` route through `crush_pkg::runners::get_runner_for_payload` for every capsule type; `cmd_build` / `cmd_check` keep `PackageBuilder` for crush and refuse non-crush via the new `require_crush_buildable` helper; `require_crush_language` is removed. Two follow-ups intentionally deferred to a future commit on top of M2: args forwarding to the runner, and a `(halted)` status line for stopped processes.

End-to-end workspace `cargo check` is independently blocked by an upstream source bug in `crush-ast/crates/crush-vm/src/portable_vm.rs` (file ends mid-function around line 1887; `mod tests` is missing its closing brace) at `crush-ast@a45d42d`. That is a separate repo, out of scope for the M2 commit; repair prerequisite for end-to-end validation. M2's `src/main.rs` alone compiles cleanly.

M1 (comprehensive test coverage, SQUEEZE-2 + SQUEEZE-3) is **the next merge-gated unit**, now sharpened by M2's landed cutover. M1's regression net must cover: dispatch correctness across capsule types (crush / native / bun / node / deno / python / sona), the literal cargo-shaped status lines (`checking <name> v<version>`, `building <name> v<version>`, `running <name>`, `done: N function(s), N byte(s)`), the documented exit codes, the two M2-deferred follow-ups, and language-archetype regression tests across M3–M4.

## Planning refresh / M2 doc-follow-up (this session)

`.jagent/planning/` board continued from the 2026-07-16 morning refresh:

- STATE.md (this update): snapshot table flipped — `Script/Native cutover (M2)` row is now `code-complete`; M1 row stays `missing` (still next merge gate)
- TASKS.md (this update): SQUEEZE-1 sub-bullets `cmd_run` / `cmd_build+cmd_check` / `require_crush_language narrowed` / `default-flow on non-crush` flipped to `[x]`; P0 "Non-`crush` `language` value" line clarified to reflect `require_crush_buildable` instead of the v0.0.1 `require_crush_language`
- SQUEEZE-1 ticket (this update): frontmatter `Status` → "Code Complete on feature/m2-script-native-cutover @ a41b546; awaiting master merge"; success-criteria checkboxes all ticked
- `.dejavue/decisions.md` (this update): new 2026-07-16 sequencing-override entry documented
- `.dejavue/state.md` / `.dejavue/handoff.md` (this update): next-steps reflect M2-merged path

The doc commit lands on `planning/m1-m6-roadmap-refresh` as a follow-up to `df0fc8a`.

## Blockers

- **Upstream (out of scope for M2):** `crush-ast@a45d42d` `crates/crush-vm/src/portable_vm.rs` ends mid-function around line 1887; `mod tests` is unclosed. Prerequisite for end-to-end workspace `cargo check`. Not this repo, not this commit.
- **Process:** none held. User is the merge authority for `feature/m2-script-native-cutover`, `planning/m1-m6-roadmap-refresh`, and `release/v0.1.0`. Best merge order: `feature/m2-script-native-cutover` first (so the docs describe the code), then `planning/m1-m6-roadmap-refresh`, then `release/v0.1.0` (long after M1/M5 lands, since the registry publish needs CI green).
