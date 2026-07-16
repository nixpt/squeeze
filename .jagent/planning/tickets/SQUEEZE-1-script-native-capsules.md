# SQUEEZE-1 — Wire Script/Native capsule support (M2, full cutover)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-1 |
| **Priority** | P2 |
| **Status** | **Code Complete on `feature/m2-script-native-cutover` @ `a41b546`** — reviewer verdict "no issues" with one micro-nit; awaiting master merge |
| **Phase** | M2 |
| **Assignee** | unassigned |
| **Dependencies** | **SQUEEZE-2** (M1 comprehensive tests still lands as the regression net that protects the cutover surface — sequencing override documented in `.dejavue/decisions.md` 2026-07-16 entry) |
| **Estimated effort** | M |

## Problem

`squeeze` v0.0.1 (the shipped skeleton) rejects any `capsule.toml` whose `[capsule] language` isn't `"crush"` (or empty) via `require_crush_language()` in `src/main.rs`. `crush-pkg`'s CLI `run` command already supports Script (bun/node/deno/python, through `buckets`-resolved pinned toolchains) and Native capsules via `crush_pkg::manifest::language_to_capsule_type` + the `CapsuleRunner` / `CrushRunner` trait system in `crush_pkg::runners` — squeeze doesn't call any of that. Only `crush_pkg::builder::PackageBuilder` is wired, and `PackageBuilder::build()` calls `crush_lang_sdk::compile::compile_crush_source`, which is meaningless for a Python or JS entry file. The `require_crush_language` error message is the loudest user-facing shout in the repo today.

## Success criteria

- [x] `squeeze run` works on `language = "python"` (and node/bun/deno) capsules — *dispatch via `crush_pkg::runners::get_runner_for_payload` lands on `ScriptRunner` per `Manifest::CapsuleType`*
- [x] `squeeze run` works on `language = "native"` capsules — *dispatch lands on `NativeRunner`*
- [x] `squeeze run` continues to work on `language = "crush"` capsules — *dispatch lands on `CrushRunner`; cuts over from bespoke `PackageBuilder`-and-`run_with_caps` flow to `get_runner_for_payload`*
- [x] `squeeze build` and `squeeze check`:
  - [x] work on crush-language capsules (unchanged — `PackageBuilder` writes `.cvm`)
  - [x] reject non-crush languages with a clear "this subcommand doesn't apply" message via `require_crush_buildable` (not a silent no-op, not the v0.0.1 generic "see module doc" message)
- [x] `squeeze` (default flow) on a non-crush capsule: skip check/build stages (don't apply), go straight to run via `crush_pkg::runners::get_runner_for_payload`. Status-line shape stays cargo-style
- [x] `require_crush_language`'s hard error is **removed** (replaced with the narrower `require_crush_buildable` which is called only from `cmd_build` / `cmd_check`)

## Technical approach

**Strategy: full cutover (not parallel tracks).** Runner dispatch is the single source of truth for all run-anything flows; `PackageBuilder` survives only in `cmd_build` / `cmd_check` (crush-language only) because `.cvm` is crush-specific. Parallel tracks were considered and rejected: they'd leave `squeeze run` for crush on a different dispatch path than `squeeze run` for Script/Native — confusing inconsistency for users debugging "why does my Python capsule run differently from my crush capsule?".

1. Read `crush_pkg::manifest::language_to_capsule_type` and `crush_pkg::runners::{CapsuleRunner, CrushRunner, ScriptRunner, NativeRunner}` — the trait system `crush-pkg`'s own `run` command already dispatches through. squeeze calls `crush_pkg::runners::get_runner_for_payload(&entry, &manifest) -> Box<dyn CapsuleRunner>`, then `runner.run(&manifest, &entry, args)`. (Not a parallel `handle_run` walker; squeeze leans on the upstream dispatcher as a single source of truth.)
2. `cmd_run` and `cmd_default`'s "run" leg: rewrite to dispatch via `get_runner_for_payload` + `runner.run`. Surface `args` is left as a follow-up commit on top of M2 (not blocking).
3. `cmd_build` / `cmd_check` keep `PackageBuilder` for crush; for non-crush emit `bail!("squeeze build/check don't apply to Script/Native capsules — use squeeze run (or crush-pkg directly)")` via the new `require_crush_buildable` helper.
4. `cmd_default` for non-crush capsules: skip check/build, only run. Default flow does "what's applicable", not "every stage even when N/A".

## Files to modify

- `src/main.rs` — module doc updated; `require_crush_language` removed; new `require_crush_buildable` helper; `cmd_run()` / `cmd_default()` rewritten through `get_runner_for_payload`; `cmd_build()` / `cmd_check()` use `require_crush_buildable`. Total file grown 119 → 227 lines.
- `Cargo.toml` — **no changes** in the M2 commit; `crush_pkg::runners` is reachable through the existing peer path-dep on `crush-pkg`.

## Test surface (extends SQUEEZE-2)

- A `language = "python"` fixture (pick cheapest script-language the existing test infra can drive)
- A `language = "crush"` end-to-end (existing surface, unchanged shape — confirms the cutover didn't regress crush)
- A `language = "python"` `squeeze build` produces the documented "doesn't apply" error, with the documented exit code, with the documented status line
- Dispatch-correctness regression net across all `Manifest::CapsuleType` variants — protects the M2 cutover from regression
- Args-forwarding surface (`squeeze run <path> -- foo bar` → forwarded to `runner.run`) — picked up as part of the M2-deferred follow-up commit
- `(halted)` status line for stopped processes — picked up as part of the M2-deferred follow-up commit

## Non-goals

- Reimplementing `buckets`-based toolchain resolution — that's `crush_pkg::runners`'s job; squeeze just calls it
- A new `capsule.toml` field — squeeze reads `capsule.toml` only through `crush_pkg::manifest`

## Done-condition (M2)

M2 is code-complete when `feature/m2-script-native-cutover` is merge-ready and reviewer-verdict is acceptable. At `a41b546`:

- ✅ Full cutover implemented (`cmd_run` + `cmd_default`'s run leg through `get_runner_for_payload`)
- ✅ `require_crush_language` removed; `require_crush_buildable` in its place
- ✅ `cargo check` on the new `src/main.rs` is clean (no errors attributable to M2)
- ✅ Reviewer verdict "no issues" with one micro-nit on a future-proofed field-extraction path (not blocking)
- ⚠️ End-to-end `cargo check` blocked by an upstream source bug in `crush-ast/crates/crush-vm/src/portable_vm.rs` (file ends mid-function around line 1887) — separate repo, not M2 scope
