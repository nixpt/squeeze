# Roadmap — squeeze

Living plan. Dejavue holds *why*; this file holds *sequence*.

## North star

One intuitive command for the whole Crush build/run/ship lifecycle —
`crush-pkg`'s pipeline underneath, `cargo`-shaped ergonomics on top. Not a
replacement for `crush-pkg`; a composition layer that grows to cover the
full capsule spectrum (crush / script / native / wasm) and the artifacts
`crush-pkg` already knows how to produce (`.cvm`, `.crush-pack`, `.ecap`).

## Current phase: M0 — ship the skeleton

The default composed flow (check → build → run) and the four explicit
subcommands (`new`/`check`/`build`/`run`) exist and are verified end-to-end
for `language = "crush"` capsules. Nothing beyond that yet.

## Milestones

| Phase | Name | Goal | Exit criteria |
|-------|------|------|----------------|
| **M0** | Ship the skeleton | `new`/`check`/`build`/`run` + default flow, crush-language only. | ✅ Verified against a real capsule. |
| **M1** | Test coverage | Real test suite, following `crush-pkg`'s own tempdir-based pattern. | Tests cover new/check/build/run + the "no capsule.toml" / "non-crush language" error paths. |
| **M2** | Script/Native capsules | Wire `crush_pkg::runners::{CapsuleRunner, CrushRunner}` instead of the `PackageBuilder`-only path. | `squeeze run` works on a `language = "python"` / `"node"` capsule, not just crush. |
| **M3** | Wasm target | `squeeze build --target wasm32` (or similar) produces a `.cvm` blob plus wires it through `crush-web`'s `run_blob(bytes)` — the two things built this session meet here. | A capsule built with `squeeze` runs unmodified in a browser via `crush-web`. |
| **M4** | Distribution commands | Fold in `crush-pkg`'s `pack`/`sign`/`site` under squeeze's simpler surface, where it makes sense. | `squeeze publish`-shaped UX, still backed by `crush-pkg`'s real signing/packing code. |

## Non-goals (standing)

- **Reimplementing crush-pkg's compiler/VM pipeline** — `PackageBuilder` and
  `crush_vm::run*` are the source of truth; squeeze only composes calls to them.
- **A competing manifest format** — `capsule.toml` stays `crush-pkg`'s format;
  squeeze reads it through `crush_pkg::manifest`, doesn't parse it itself.
- **Becoming the toolchain installer** — that's `crush-installer`'s job.
