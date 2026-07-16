# Decisions


## 2026-07-14T19:22:53-05:00 — [STRATEGIC] [ADOPTED] [ARCHITECTURAL] squeeze: new peer repo wrapping crush-pkg, named after crush's juice/pressing metaphor

Reason:
crush-pkg build and crush-pkg run are two separate, uncomposed steps today — run interprets the entry source directly via CapsuleRunner/PayloadFormat detection, it never calls build() or writes target/. squeeze's whole value in v0.1 is composing check->build->write target/->run into one command with one status line per stage, the way 'cargo run' does, without reimplementing any of crush-pkg's actual compile/run pipeline (PackageBuilder is the real work; squeeze is a thin composition layer).

Artifacts: src/main.rs, Cargo.toml

Author type: agent

Rejected alternatives:
- **bob / bob-the-builder**: heavily conflicted name — BobBuildTool/bob (embedded/bitbake-style), benchkram/bob (multi-language monorepo build tool, conceptually near-identical to this), bobbin-cli (embedded Rust deploy), a published 'bob' crate on crates.io. Confirmed via web search before naming.
- **crates/squeeze inside crush-ast's own workspace**: would inherit the same feature-unification headaches CRUSHAST-WEB-1 (crush-web) just spent real effort untangling, and squeeze is meant to grow into an ecosystem-wide tool (potentially touching buckets/crush-web/exo-light later), not something crush-ast-workspace-scoped. Standalone peer repo matches the established pattern for new ecosystem tools this session (buckets, interactd, crush-vscode).

Outcome:
v0.1 skeleton: squeeze (default: check+build+run in one flow) / new / check / build / run subcommands, thin wrapper over crush_pkg::builder::PackageBuilder + crush_vm::run_with_caps. Verified end-to-end against a real 'squeeze new demo' -> target/demo.cvm + demo.casm.json -> 'hello from Crush' output, both via the default flow and via 'squeeze run' with no prior build.



## 2026-07-16T11:00:00-05:00 — [STRATEGIC] [ADOPTED] [ARCHITECTURAL] squeeze: M1 comprehensive tests before M2 full-cutover; M5 CI+publishing between M2 and M3; new M6 v1.0 CLI stability contract

Reason:
M2 (SQUEEZE-1) is the most-asked gap and rewrites the load path; doing it without tests means regressions in runner dispatch silently swallow non-crush capsules. M1 (comprehensive tests + CI in M5) MUST land first as the regression net. M1 scope is comprehensive — clap args, status-line literals, exit codes are captured now because they become M6's stability contract retroactively; cheaper to capture once than retrofit.

M2 dispatch strategy is full cutover (not parallel tracks). Parallel tracks were considered and rejected: they'd leave `squeeze run` for crush on a different dispatch path than `squeeze run` for Script/Native, which is a confusing inconsistency for users debugging "why does my Python capsule run differently from my crush capsule?". Single dispatch path via `crush_pkg::runners::handle_run()` is easier to reason about; M1 tests make the flight safer.

M5 (CI + crates.io publishing) sits between M2 and M3 without CI the M1 tests don't run anywhere, and without publishing downstream consumers can't `cargo install squeeze`. M3 (wasm/crush-web) is mostly verification + documentation, not new code; lands late so it integrates against a published `squeeze`. M6 (v1.0 CLI stability) is the natural close — factor status-line literals into `messages.rs`, type exit codes, capture surface in MANUAL.md, write CHANGELOG.md for v0.x → v1.0.

Artifacts:
- .jagent/planning/ROADMAP.md (rewritten: M0–M6 with explicit exit criteria)
- .jagent/planning/TASKS.md (rewritten: sub-tasks bucketed by milestone)
- .jagent/planning/STATE.md (refreshed: M1 is the next merge-gated unit)
- .jagent/planning/tickets/SQUEEZE-1-script-native-capsules.md (updated: full-cutover strategy, narrowed require_crush_language scope)
- .jagent/planning/tickets/SQUEEZE-2-comprehensive-test-coverage.md (new: M1)
- .jagent/planning/tickets/SQUEEZE-3-ci-and-crates-io.md (new: M5)
- .jagent/planning/tickets/SQUEEZE-4-wasm-crush-web-integration.md (new: M3, expected doc-only)
- .jagent/planning/tickets/SQUEEZE-5-distribution-commands.md (new: M4)
- .jagent/planning/tickets/SQUEEZE-6-v1-cli-stability-contract.md (new: M6)

Rejected alternatives:
- **M2 parallel-tracks (Option A)**: keep `require_crush_language` for crush-language, add a separate `CapsuleRunner` dispatch for Script/Native. Considered because it preserves the bespoke `build` step cleanly; rejected because it leaves `squeeze run` inconsistent across capsule types — debugging "why does the same `squeeze` behave differently between my .crush and .py capsules" is a worse UX than the cleaner cutover.
- **M1 minimal regression net (vs comprehensive)**: only test the crush-language path narrowly. Considered because it's smaller work; rejected because M1 tests are also M6's contract in test form — capturing the run-once surface now avoids retro-fitting every contributor's "while I'm here, I'll just tidy this println" later.
- **M2 feature-first (M2 before M1)**: land SQUEEZE-1 first since it's been waiting the longest. Considered because it's the most-asked gap; rejected because without M1 tests, regressions in the runner-dispatch refactor are silent — we already know `require_crush_language` is one of the loudest user-facing shouts, we don't need a second one about "why does my Wasm capsule quietly produce nothing".

Outcome:
Roadmap is now: M0 (shipped) → M1 (comprehensive tests + CI) → M2 (full-cutover Script/Native) → M5 (crates.io publish) → M3 (wasm/crush-web doc verification) → M4 (pack/sign distribution wrappers) → M6 (v1.0 CLI stability contract). Branch `planning/m1-m6-roadmap-refresh` carries the planning deltas; merge-at-user-authority.



## 2026-07-16T13:30:00-05:00 — [STRATEGIC] [ADOPTED] [SEQUENCING-OVERRIDE] M2 implemented ahead of M1 sequencing; M1 still lands as SQUEEZE-2

Reason:
The 2026-07-16 morning decision said M1 (comprehensive tests) MUST land before M2 (the full cutover). On 2026-07-16 afternoon the user explicitly invoked "lets start m2", overriding the ordering for the rest of this session. Rationale that the override was sensible: the M2 cutover is a small, human-reviewable surface (a single Rust file, `src/main.rs`, grown 119 → 227 lines; one dispatch path through `crush_pkg::runners::get_runner_for_payload`; one per-subcommand guard `require_crush_buildable`), and M0's existing end-to-end `cargo check` exercise plus the M1 SQUEEZE-2 ticket both already document the surface tests that M2 needs. M1 still lands because SQUEEZE-2 — comprehensive tests + CI — covers the broader roadmap (status-line literal tests, exit-code tests, language-archetype tests across M3–M4 follow-ups, and the two M2-deferred follow-ups: args forwarding, halted status). This entry exists so a future session finds the rationale and treats the override as a deliberate, time-boxed move rather than drift to repeat.

Artifacts:
- `src/main.rs` @ `feature/m2-script-native-cutover` `a41b546` (119 → 227 lines; full cutover)
- `.dejavue/state.md` (this commit: M2 code-complete snapshot)
- `.dejavue/handoff.md` (this commit: M2 status + deferred follow-ups + upstream blocker)
- `.jagent/planning/STATE.md` (this commit: snapshot table flipped; M2 code-complete; M1 still next merge gate)
- `.jagent/planning/TASKS.md` (this commit: SQUEEZE-1 sub-bullets checked; P0 non-crush line clarified)
- `.jagent/planning/tickets/SQUEEZE-1-script-native-capsules.md` (this commit: status → "Code Complete on feature/m2-script-native-cutover"; success criteria all ticked)
- `release/v0.1.0` @ `d42710a` (unchanged this commit; merges separately)
- `master` @ `63cb23d` (unchanged)

Rejected alternatives:
- **Defer M2 until M1 lands as first planned**: refused because that's exactly the 2026-07-16 morning decision the user overrode. Documenting, not re-litigating.
- **Land M2 ahead of M1 without documenting the override**: refused because state-mutation without rationale is the failure mode that produces "wait, when did we stop testing" tomorrow. This entry's job is to make the override visible to the next session.

Outcome:
Per-session sequence so far: M0 → M2. M1 remains on the queue as SQUEEZE-2 (next merge-gated unit), now with a sharper target because M2 has landed the cutover surface — the M1 regression net must specifically cover dispatch correctness across capsule type, the literal status lines, the two deferred M2 follow-ups, and the exit codes documented in RELEASE.md.
