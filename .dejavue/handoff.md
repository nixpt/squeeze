# Handoff

Updated: 2026-07-16T13:30:00-05:00

## Summary

M2 (SQUEEZE-1, full cutover for Script/Native) is **code-complete on `feature/m2-script-native-cutover`** at commit `a41b546`. `src/main.rs` is the only Rust file touched in the diff. Runner dispatch is the single source of truth for every run path; `PackageBuilder` survives only inside `cmd_build` / `cmd_check` (crush-language only). Code-reviewer verdict: **"no issues"** with one micro-nit. The current doc update (this file + `.dejavue/state.md` + new strategic-decision entry in `.dejavue/decisions.md`, plus updated `STATE.md` / `TASKS.md` and the SQUEEZE-1 ticket under `.jagent/planning/`) lands as a follow-up commit on `planning/m1-m6-roadmap-refresh`.

End-to-end workspace `cargo check` is independently blocked by an upstream source bug in `crush-ast/crates/crush-vm/src/portable_vm.rs` (`mod tests` is missing its closing brace around line 1887) at `crush-ast@a45d42d`. That bug is in a separate repo, not in this M2 commit's scope; repairing it is a prerequisite for an end-to-end test run on the M2 changeset.

Other branches unchanged this session: `release/v0.1.0` @ `d42710a` (registry-pure prep); `master` @ `63cb23d` (untouched).

## Deferred within M2

Two follow-ups intentionally left for a future commit on top of M2 (not blockers, called out in the M2 commit message):

- **Args forwarding.** `cmd_run` passes an empty `&[String]` slice to `runner.run` today. `squeeze run <path> -- foo bar` will not surface `foo` and `bar` to the run. Tracked as a follow-up on `feature/m2-script-native-cutover`; surface attribute is `"args"` end-to-end so the test surface is straightforward once the regression net lands.
- **`(halted)` status line.** The `ExecutionResult::Process(Child)` arm of the dispatch does not implement a `wait()` follow-up status. Today the dispatch just reports "running for … capsule language = …". Tracked as a follow-up.

## Next Steps

- **Repair upstream `crush-ast/crates/crush-vm/src/portable_vm.rs`** so the workspace `cargo check` runs end-to-end. Without that, M2 (and every future Rust change) lands "code-complete" with an unverified dependency graph.
- **Merge `feature/m2-script-native-cutover` into `master`** (user is the merge authority). Branch carries the M2 cutover (single Rust file: `src/main.rs`).
- **Merge `planning/m1-m6-roadmap-refresh` into `master`** (user is the merge authority). Best done after the M2 merge so the docs describe what's actually on master; this commit refreshes state so the docs match M2-merged reality.
- **Start M1 / SQUEEZE-2** as the next merge-gated unit. M1's tests now cover the broader net: dispatch correctness across language archetypes, the literal status lines (cargo-shaped `checking …` / `building …` / `running …` / `done: N function(s), N byte(s)` and the default-flow staged variants), the documented exit codes (success=0, runtime=1, parse=2, version=differently-typed, manifest-missing), plus the two M2 deferred follow-ups (args forwarding, halted status).
- **Start M5 / SQUEEZE-3** alongside M1 (CI + crates.io publishing) once M1 tests exist. Tag-driven release workflow.
- **M3 / SQUEEZE-4** lands after publish: verify (likely doc-only) that `squeeze build`'s `.cvm` round-trips through `crush-web`'s `run_blob`. Document the recipe in a "Web target" section.
- **M4 / SQUEEZE-5**: wrap `crush-pkg`'s `pack`/`sign`/`verify` as `squeeze pack`/`sign`/`verify`.
- **M6 / SQUEEZE-6** at the end: factor status-line literals into `src/messages.rs`, type exit codes (`SqueezeExit`), capture the surface in `MANUAL.md`, write `CHANGELOG.md` for v0.x → v1.0, gate breaking changes post-v1.0.

## Boot Instructions

Read `.dejavue/handoff.md`, `.dejavue/state.md`, `.dejavue/decisions.md`, and `.dejavue/timeline.jsonl` before making changes. Run `dejavue context` if installed.
