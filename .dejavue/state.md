# State

Updated: 2026-07-16T13:30:00-05:00

M2 (SQUEEZE-1, full cutover for Script/Native) is **code-complete on `feature/m2-script-native-cutover`** at commit `a41b546`. `src/main.rs` rewritten (119 → 227 lines): `cmd_run` and the run leg of `cmd_default` route through `crush_pkg::runners::get_runner_for_payload` for every capsule type (crush / native / bun / node / deno / python / sona); `cmd_build` / `cmd_check` keep `PackageBuilder` for crush-language capsules and refuse non-crush with a clear "doesn't apply" message via the new `require_crush_buildable` helper; the v0.0.1 `require_crush_language` hard error is removed. `cargo check` on the new `src/main.rs` is clean.

End-to-end workspace `cargo check` is independently blocked by an upstream source bug in `crush-ast/crates/crush-vm/src/portable_vm.rs` (the file ends mid-function around line 1887 — `mod tests` is missing its closing brace) at `crush-ast@a45d42d`. That is a separate repo's source and not in scope for the M2 commit. Repairing it is a prerequisite for end-to-end test validation against the M2 changeset.

Code-reviewer verdict on `a41b546`: **"no issues"** with one micro-nit on a future-proofed field-extraction path (not blocking).

Per-session sequence was M0 → **M2**, with M1 deliberately sequenced forward by user direction this session; M1 still lands as SQUEEZE-2 because its tests cover the broader roadmap (status-line literals, exit codes, language-archetype tests across M3–M4 follow-ups). The override rationale is captured in `.dejavue/decisions.md` (2026-07-16 sequencing-override entry) so a future session treats it as a deliberate, time-boxed move rather than drift to repeat.

These doc updates land as a follow-up commit on `planning/m1-m6-roadmap-refresh`. Other branches: `release/v0.1.0` @ `d42710a` (registry-pure prep, unmerged); `master` @ `63cb23d` (untouched). User is the merge authority for both `feature/m2-script-native-cutover` and `planning/m1-m6-roadmap-refresh`.
