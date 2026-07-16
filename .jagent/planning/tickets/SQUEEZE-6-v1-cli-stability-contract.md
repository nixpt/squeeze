# SQUEEZE-6 — Ratify the v1.0 CLI stability contract (M6)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-6 |
| **Priority** | P3 |
| **Status** | Backlog |
| **Phase** | M6 |
| **Assignee** | unassigned |
| **Dependencies** | SQUEEZE-2 + SQUEEZE-3 (tests + CI must enforce the contract before it gets codified) |
| **Estimated effort** | S (mechanical capture + CHANGELOG) but release-quality — changes the UX downstream depends on |

## Problem

`squeeze` aims to be an intuitive one-command build tool — that means downstream tooling (editor integrations, runtimes like `crush-notebook`, CI scripts that parse `squeeze`'s output) can reasonably depend on its CLI surface. But the surface today is informal — status-line formats, exit codes, and error shapes live inline in `src/main.rs` and shift if a contributor "tidy up" a `println!`. M6 ratifies the contract from what's already there, so a future rewrite is a breaking-change event with a migration note (not a silent regression).

## Success criteria

- [ ] `MANUAL.md` (or `docs/cli.md`) lists, per subcommand:
  - [ ] Synopsis
  - [ ] All flags with documented semantics
  - [ ] Documented exit codes (success=0, runtime/panic=1, parse/manifest=2, wrong-language=3, missing-manifest=4 — these are M1 captures, ratified here)
  - [ ] The literal status-line format string per stage (matching the M1 tests' expected text verbatim)
  - [ ] Documented error message templates
- [ ] `CHANGELOG.md` exists from v0.x → v1.0, capturing any breaking UX changes made during M1–M5 (most likely: the M2 narrow of `require_crush_language`)
- [ ] Post-v1.0 breaking-change gate: any PR that changes a documented exit code, status-line literal, or subcommand surface shape MUST add a CHANGELOG entry under "Unreleased / Breaking" before merging
- [ ] README links to `MANUAL.md`

## Technical approach

1. Pull the literal status-line strings out of `src/main.rs` into a small `messages.rs` module. Each becomes `pub const fn checking(name: &str, version: &str) -> String` etc.
2. Pull the exit codes into `pub enum SqueezeExit { Ok, Runtime, ManifestParse, WrongLanguage, MissingManifest }` with `impl Into<i32> for SqueezeExit`; replace all bare `Ok(())` / `bail!` callsites with the typed exit.
3. Capture the resulting surface in `MANUAL.md`. The existing test suite (SQUEEZE-2) is the contract's source — the manual is the human-readable mirror.
4. `CHANGELOG.md` from v0.1.0 → v1.0.0.
5. CI lint (custom cargo deny rule? simple grep check? `cargo insta` snapshot?) that flags any change to `messages.rs` outside an "expect-tests-fail too" commit message trigger — TBD per CI maturity.

## Files to modify

- `src/main.rs` — refactor to use `messages.rs` and the typed exit enum
- New `src/messages.rs` — literal status-line strings + `SqueezeExit`
- New `MANUAL.md` (or `docs/cli.md`)
- New `CHANGELOG.md`

## Non-goals

- Auto-generating `MANUAL.md` from clap derives (`clap_markdown` later; out of scope for v1.0 capture)
- Backwards-compat shims for any pre-v1.0 callers — squeeze is pre-1.0; the contract locks in *at* v1.0, not retroactively
- A machine-readable JSON output mode (defer until user-requested)
