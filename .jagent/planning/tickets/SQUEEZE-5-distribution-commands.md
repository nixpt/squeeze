# SQUEEZE-5 — Wrap `crush-pkg`'s distribution commands (M4)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-5 |
| **Priority** | P2 |
| **Status** | Backlog |
| **Phase** | M4 |
| **Assignee** | unassigned |
| **Dependencies** | SQUEEZE-1 (M2 manifest loading + runner dispatch stable), SQUEEZE-2 (M1 tests) |
| **Estimated effort** | M |

## Problem

`crush-pkg` exposes a `pack` / `sign` / `verify` / `site` / `site-extract` surface for capsule distribution (publishing `.crush-pack`, signing, generating the static-site manifest). `squeeze` doesn't expose those — users still have to `crush-pkg pack` and `squeeze build` for two separate concerns. The north star is "one intuitive command for the whole build/run/ship lifecycle"; ship is the missing third leg.

## Success criteria

- [ ] `squeeze pack` — thin pass-through to `crush_pkg::ops::pack(&manifest, &target)`. Writes `.crush-pack` to the same `target/` `squeeze build` already uses.
- [ ] `squeeze sign` — thin pass-through to `crush_pkg::ops::sign(&pack_path, &key)`. Writes `.ecap` next to `.crush-pack`.
- [ ] `squeeze verify` — thin pass-through to `crush_pkg::ops::verify(&ecap_path, &key)`. Exits 0 / non-zero per `crush-pkg`'s contract.
- [ ] (Maybe) `squeeze site` & `squeeze site-extract` — defer if `crush-pkg`'s site commands aren't stable yet; revisit at M6.
- [ ] All subcommands share `squeeze`'s status-line format ("packing <name> v<version>", "done: <size> bytes", etc.)
- [ ] All subcommands have tests (reuse SQUEEZE-2's harness)

## Technical approach

1. Read `crush_pkg::ops::{pack, sign, verify}` — they're the source of truth; squeeze is the surface.
2. Add three (or five) subcommand arms to `Cli`'s `Commands` enum. Status-line strings per SQUEEZE-6's documented format.
3. Factor common manifest loading out of `cmd_default` so `pack` / `sign` reuse it instead of duplicating tempdir + scaffold logic.
4. Tests: scaffold a real capsule, `squeeze build` → `squeeze pack` → assert `.crush-pack` exists, then `squeeze sign` → assert `.ecap`, then `squeeze verify` → assert exit 0. The full chain is the regression net for the whole distribution surface.

## Files to modify

- `src/main.rs` — three (or five) new subcommand arms, plus a small factoring refactor of `load_capsule`

## Non-goals

- Reimplementing pack/sign/verify primitives — `crush_pkg::ops::*` is the source of truth
- A new key-management UX (key generation, key rotation) — out of scope; squeeze invokes whatever key the caller passes
- `squeeze publish` — out of scope until we know which registry (`crushhub`? crates.io? something new?) is the right home for `.ecap` distribution. Revisit at M6.
