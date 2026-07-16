# Roadmap — squeeze

Living plan. Dejavue holds *why*; this file holds *sequence*.

## North star

One intuitive command for the whole Crush build/run/ship lifecycle — `crush-pkg`'s pipeline underneath, `cargo`-shaped ergonomics on top. Not a replacement for `crush-pkg`; a composition layer that grows to cover the full capsule spectrum (crush / script / native / wasm) and the artifacts `crush-pkg` already knows how to produce (`.cvm`, `.crush-pack`, `.ecap`).

## Current phase: M1 — test coverage (comprehensive)

M0 (skeleton) shipped 2026-07-14 and verified against a real `squeeze new demo` -> `target/demo.cvm` flow. M1 is the active phase. Strategy ratified this session (see `.dejavue/decisions.md`):

- **M1 (tests) before M2 (Script/Native)**: lock the regression net before changing the load path.
- **M1 scope = comprehensive**: every `clap` argument, status-line format, exit code, and the documented error paths are tested.
- **M2 = full cutover**: `cmd_run` and the default flow route through `crush_pkg::runners::handle_run()` for every capsule type; crush-language `build`/`check` still call `PackageBuilder` for the `.cvm` artifact (crush-specific).

## Milestones

| Phase | Name | Goal | Exit criteria |
|-------|------|------|----------------|
| **M0** | Ship the skeleton | `new` / `check` / `build` / `run` + default composed flow, crush-language only. | Verified against a real capsule (see STATE). |
| **M1** | Test coverage (comprehensive) | Real test suite plus CI so every clap arg, every status-line format, every documented exit code, and every documented error path is exercised on every push. | GitHub Actions green on master; `cargo test` covers new/check/build/run/default flow + `require_crush_language` refusals + clap subcommand shape; status-line strings are asserted verbatim. |
| **M2** | Script/Native capsule support (full cutover) | `squeeze run` and the default flow route through `crush_pkg::runners::handle_run()` for every capsule type. Crush-source `build`/`check` stay on `PackageBuilder` for `.cvm`. | A capsule with `language = "python"` (or node/bun/deno) runs via `squeeze` end-to-end; the existing crush path still passes its M1 tests; `require_crush_language` either goes away or narrows to just `build`/`check`. |
| **M3** | Wasm target + crush-web integration | Confirm (and document) that `crush-web`'s `run_blob(bytes)` consumes whatever `.cvm` `squeeze build` already writes — likely no new squeeze code, just a verified example. | Example capsule built with `squeeze` runs unmodified inside a browser via `crush-web`; README captures the recipe. |
| **M4** | Distribution commands | Wrap `crush-pkg`'s `pack` / `sign` / `verify` under `squeeze`'s cargo-shaped surface. | `squeeze pack` / `squeeze sign` / `squeeze verify` are thin pass-throughs to `crush_pkg::ops::{pack, sign, verify}` and share `squeeze`'s status-line format. |
| **M5** | CI + crates.io publishing | The M1 tests run on every PR; the published `squeeze` crate is on crates.io with a tag-driven release pipeline. | GitHub Actions: `cargo test` on stable, lockfile audit, deny advisories. `cargo publish` dry-run succeeds. crates.io metadata, keywords, README render look right. |
| **M6** | v1.0 CLI stability contract | The CLI surface — subcommands, flags, exit codes, status-line format, error format — is locked as a public contract and documented in `MANUAL.md` or equivalent. | `MANUAL.md` lists every subcommand, every flag, every documented exit code, the literal status-line format strings. `CHANGELOG.md` exists from v0.x → v1.0; breaking changes post-v1.0 require a migration note. |

## Non-goals (standing)

- **Reimplementing crush-pkg's compiler/VM pipeline** — `PackageBuilder`, `crush_vm::run*`, and (post-M2) `crush_pkg::runners::handle_run` are the source of truth; squeeze only composes calls.
- **A competing manifest format** — `capsule.toml` stays `crush-pkg`'s format; squeeze reads it through `crush_pkg::manifest`, never parses itself.
- **Becoming the toolchain installer** — that's `crush-installer`'s job; squeeze just runs what `crush-pkg` already knows how to run.
- **Telemetry, plugin systems, result-cache** — out of scope for v0.x/v1.0. Re-evaluate post-v1.0.
- **Multi-crate / workspace build support** — `squeeze` operates on one `capsule.toml` at a time.

## Sequencing rationale

The most-asked gap is M2 — `crush-pkg` supports other capsule types, `squeeze` doesn't (the `require_crush_language` error message is the loudest user-facing shout in the repo). But M2 rewrites the load path; without tests a regression in dispatch silently mis-runs non-crush capsules. So M1 lands first as the safety net. M1 scope is comprehensive because the public CLI contract (exit codes, status lines, error shapes) becomes the basis for M6's stability promise, and it's cheaper to capture once than to retrofit.

Full cutover in M2 (not parallel tracks) because parallel-tracks would leave `squeeze run` for crush on a different dispatch path than `squeeze run` for Script/Native — confusing inconsistency for users debugging "why does my Python capsule run differently from my crush capsule?". One dispatch path is easier to reason about; M1 tests make the flight safer.

M5 (CI + publishing) sits between M2 and M3/M4 because without CI the M1 tests don't run anywhere; without publishing downstream consumers (crush-web integration recipes, internal Crush projects) can't `cargo install squeeze`. M3 (wasm / crush-web) is mostly verification + documentation, lands late. M6 is the natural close — once features settle, lock the CLI as a contract so downstream tooling (editor integrations, runtimes) can depend on exit codes and status-line formats without re-reading source every release.
