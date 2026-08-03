# SQUEEZE-7 — Remove dead `crush-vm` dependency

**Status**: Backlog · **Priority**: P3 (hygiene)

## Problem

`Cargo.toml:32` declares `crush-vm` 0.3.0 with ZERO usage anywhere in src/
(verified by the crush-ast CRUSH-71 client survey, s412 2026-08-02: squeeze's
real surface is `crush_pkg::{Manifest, PackageBuilder, runners, manifest}`).
The dead dep inflates build time and every "who consumes crush-vm" ripple
analysis. Re-verify with a grep before removing (negative grep ≠ absence —
check feature-gated code too).

## Done

- [ ] Dep removed; `cargo check` + `cargo test` green
- [ ] Cross-ref: crush-ast ticket CRUSH-86 (tracks this + the
      crush-visuals-debug-bridge twin) — note completion there
