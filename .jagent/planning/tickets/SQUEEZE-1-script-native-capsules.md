# SQUEEZE-1 — Wire Script/Native capsule support

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-1 |
| **Priority** | P2 |
| **Status** | Backlog |
| **Phase** | M2 |
| **Assignee** | unassigned |
| **Dependencies** | none |
| **Estimated effort** | M |

## Problem

`squeeze`'s v0.1 `load_capsule()` rejects any `capsule.toml` whose
`[capsule] language` isn't `"crush"` (or empty) via `require_crush_language`
in `src/main.rs`. `crush-pkg`'s own CLI `run` command already supports
Script (bun/node/deno/python, via `buckets`-resolved pinned toolchains) and
Native capsules through `crush_pkg::manifest::CapsuleType` and the
`CapsuleRunner`/`CrushRunner` trait system in `crush_pkg::runners` — squeeze
just doesn't call into any of that yet, it only drives
`crush_pkg::builder::PackageBuilder` (which is crush-language-specific:
`build()` calls `crush_lang_sdk::compile::compile_crush_source`, meaningless
for a Python/JS entry file).

## Success criteria

- [ ] `squeeze run` works on a `language = "python"` (or node/bun/deno) capsule
- [ ] `squeeze run` works on a `language = "native"` capsule
- [ ] `squeeze build`/`check`/the default flow either support these too, or
      fail with a clear "not applicable to Script/Native capsules" message
      (not a silent no-op) — decide per-subcommand, don't assume symmetry
      with the crush-language path
- [ ] `require_crush_language`'s error message either goes away entirely or
      narrows to whatever's still genuinely unsupported

## Technical approach

1. Read `crush_pkg::manifest::language_to_capsule_type` +
   `crush_pkg::runners::{CapsuleRunner, CrushRunner}` — this is the exact
   trait system `crush-pkg`'s own `handle_run()` (`main.rs`) already
   dispatches through; squeeze should reuse it rather than inventing a
   parallel path.
2. `PackageBuilder`/`build()`/`check()` stay as the crush-language-specific
   path (they're for the `.cvm` artifact, which only means something for
   crush source) — Script/Native likely route around them entirely for
   `run`, matching how `crush-pkg`'s CLI already does it.
3. Re-check whether the default composed flow (`squeeze` with no
   subcommand) even makes sense for a Script/Native capsule — "build" may
   not be a meaningful step for e.g. a Python entry file.

## Files to modify

- `src/main.rs` — `load_capsule()`, `require_crush_language()`, `cmd_run()`, `cmd_default()`

## Non-goals

- Reimplementing `buckets`-based toolchain resolution — that's `crush_pkg::runners`'s job, squeeze just calls it
