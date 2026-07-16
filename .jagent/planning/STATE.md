# Planning state — squeeze

**Updated:** 2026-07-16
**Milestone focus:** M1 — test coverage (comprehensive)
**Branch:** M0 on `master`; planning refresh lives on `planning/m1-m6-roadmap-refresh` (worktree at `../squeeze--planning-m1-m6`, unmerged).

## Delivery snapshot

| Track | Status | Notes |
|-------|--------|--------|
| CLI skeleton (M0) | shipped | `new` / `check` / `build` / `run` + default flow; verified end-to-end. |
| PackageBuilder integration (M0) | shipped | Peer path-dep; no reimplementation of compile/run. |
| Tests (M1) | missing | SQUEEZE-2 backlog; comprehensive scope ratified 2026-07-16. |
| CI (M5) | missing | SQUEEZE-3 backlog; lands with SQUEEZE-2 as "test infrastructure includes CI". |
| Script/Native capsules (M2) | missing | SQUEEZE-1; full-cutover strategy ratified 2026-07-16. |
| Wasm/crush-web (M3) | missing | SQUEEZE-4; expected mostly doc/example work post-publish. |
| Distribution (M4) | missing | SQUEEZE-5; wraps `crush-pkg`'s `pack`/`sign`/`verify`. |
| crates.io publishing (M5) | repo live | `nixpt/squeeze` is public; crate not yet on the registry. |
| v1.0 CLI stability contract (M6) | missing | SQUEEZE-6; locks the contract retroactively from what's already there. |
| Directory move | shipped | `projects/crush-workspace/` relocation completed 2026-07-14. |

## Active work

M1 (comprehensive test coverage, SQUEEZE-2 + SQUEEZE-3) is the next PR-merge-able unit. Pre-requisite to M2 (the actual feature work everyone is waiting for). Merge gate: M1's tests + CI are green on master.

## Planning refresh (this session)

The `.jagent/planning/` board was refreshed 2026-07-16 on branch `planning/m1-m6-roadmap-refresh`:

- ROADMAP.md now defines M0–M6 with explicit exit criteria
- TASKS.md now buckets sub-tasks by milestone
- Five new tickets authored: SQUEEZE-2..6
- SQUEEZE-1 technical approach updated for full-cutover
- `.dejavue/decisions.md` gained a new strategic decision: M1 comprehensive + M2 full-cutover, M1 before M2

The branch carries planning deltas only, no Rust yet.

## Blockers

None.
