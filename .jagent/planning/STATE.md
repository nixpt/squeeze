# Planning state — squeeze

**Updated:** 2026-07-14
**Milestone focus:** M0 — ship the skeleton
**Branch:** `master`

## Delivery snapshot

| Track | Status | Notes |
|-------|--------|--------|
| CLI skeleton | **shipped** | `new` / `check` / `build` / `run` + default composed flow (check→build→run) |
| `PackageBuilder` integration | **shipped** | Peer path-dep on `../../crush-ast/crates/crush-pkg`; no reimplementation of compile/run |
| End-to-end verification | **shipped** | Real `squeeze new demo` → `target/demo.cvm` + `.casm.json` → `hello from Crush`, both via default flow and `squeeze run` with no prior build |
| Tests | **missing** | None yet — `crush-pkg`'s own test suite (tempdir + `scaffold_package` + assert on `target/`) is the model to follow |
| Script/Native capsules | **missing** | Only `language = "crush"` wired; errors explicitly on anything else (not silent) |
| CI | **missing** | None yet |
| Publishing | **repo live** | `nixpt/squeeze` created and pushed 2026-07-14 (public). Not yet published to crates.io. |
| Directory move | **shipped** | Moved into `projects/crush-workspace/` 2026-07-14 alongside crush-ast's other consumer repos; path-deps updated in place |

## Active work

None — v0.1 skeleton just landed. Next session should read `TASKS.md` and
pick up `SQUEEZE-1` (Script/Native capsule support) or start on tests.

## Blockers

None.
