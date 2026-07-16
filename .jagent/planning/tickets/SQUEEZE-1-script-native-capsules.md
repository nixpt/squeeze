# SQUEEZE-1 — Wire Script/Native capsule support (M2, full cutover)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-1 |
| **Priority** | P2 |
| **Status** | In Progress (code landed on feature/m2-script-native-cutover; cargo check blocked by upstream crush-vm peer-crate source error — see commit message for details) |
| **Phase** | M2 |
| **Assignee** | unassigned |
| **Dependencies** | **SQUEEZE-2** (M1 comprehensive tests must land first — load-path rewrite needs a regression net) |
| **Estimated effort** | M |

## Problem

`squeeze` v0.0.1 (the shipped skeleton) rejects any `capsule.toml` whose `[capsule] language` isn't `"crush"` (or empty) via `require_crush_language()` in `src/main.rs`. `crush-pkg`'s CLI `run` command already supports Script (bun/node/deno/python, through `buckets`-resolved pinned toolchains) and Native capsules via `crush_pkg::manifest::language_to_capsule_type` + the `CapsuleRunner` / `CrushRunner` trait system in `crush_pkg::runners` — squeeze doesn't call any of that. Only `crush_pkg::builder::PackageBuilder` is wired, and `PackageBuilder::build()` calls `crush_lang_sdk::compile::compile_crush_source`, which is meaningless for a Python or JS entry file. The `require_crush_language` error message is the loudest user-facing shout in the repo today.

## Success criteria

- [ ] `squeeze run` works on `language = "python"` (and node/bun/deno) capsules
- [ ] `squeeze run` works on `language = "native"` capsules
- [ ] `squeeze run` continues to work on `language = "crush"` capsules — cuts over from bespoke dispatch to `handle_run()`
- [ ] `squeeze build` and `squeeze check`:
  - [ ] work on crush-language capsules (unchanged — `PackageBuilder` writes `.cvm`)
  - [ ] reject non-crush languages with a clear "this subcommand doesn't apply" message (not a silent no-op, not the v0.0.1 generic "see module doc" message)
- [ ] `squeeze` (default flow) on a non-crush capsule: skip check/build stages (don't apply), go straight to run via `handle_run()`. Status-line shape stays cargo-style
- [ ] `require_crush_language`'s hard error narrows to just `cmd_build` / `cmd_check` (or is replaced by per-subcommand error messages there)

## Technical approach

**Strategy: full cutover (not parallel tracks).** Runner dispatch is the single source of truth for all run-anything flows; `PackageBuilder` survives only in `cmd_build` / `cmd_check` (crush-language only) because `.cvm` is crush-specific. Parallel tracks were considered and rejected: they'd leave `squeeze run` for crush on a different dispatch path than `squeeze run` for Script/Native — confusing inconsistency for users debugging "why does my Python capsule run differently from my crush capsule?".

1. Read `crush_pkg::manifest::language_to_capsule_type` and `crush_pkg::runners::{CapsuleRunner, CrushRunner}` — the trait system `crush-pkg`'s own `handle_run()` already dispatches through. squeeze calls `crush_pkg::runners::handle_run()` directly, not a parallel dispatch walker.
2. `cmd_run` and `cmd_default`'s "run" leg: rewrite to `crush_pkg::runners::handle_run(&manifest, &entry_source, &quotas)`. If `handle_run` returns an error, wrap with a `squeeze`-prefixed message for discoverability.
3. `cmd_build` / `cmd_check` keep `PackageBuilder` for crush; for non-crush emit `bail!("squeeze build/check don't apply to Script/Native capsules — use squeeze run (or crush-pkg directly) on them")`. Narrows `require_crush_language` to those two subcommands.
4. `cmd_default` for non-crush capsules: skip check/build, only run. Default flow does "what's applicable", not "every stage even when N/A".

## Files to modify

- `src/main.rs` — `load_capsule()`, `require_crush_language()`, `cmd_run()`, `cmd_default()`, `cmd_build()`, `cmd_check()` (the latter two gain a "wrong-language" branch)
- `Cargo.toml` — only if `crush_pkg::runners` needs additional feature gate; check before bumping

## Test surface (extends SQUEEZE-2)

- A `language = "python"` fixture (pick cheapest script-language the existing test infra can drive)
- A `language = "crush"` end-to-end (existing surface, unchanged shape — confirms the cutover didn't regress crush)
- A `language = "python"` `squeeze build` produces the documented "doesn't apply" error, with the documented exit code, with the documented status line

## Non-goals

- Reimplementing `buckets`-based toolchain resolution — that's `crush_pkg::runners`'s job; squeeze just calls it
- A new `capsule.toml` field — squeeze reads `capsule.toml` only through `crush_pkg::manifest`
