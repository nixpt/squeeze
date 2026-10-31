# squeeze

Crush's cargo (SQUEEZE-8): the user-facing front door; `crush-pkg` is the
engine underneath and squeeze never reimplements its pipeline. Anything
squeeze needs that touches Crush code goes into crush-pkg (crush-ast) first. See
`.jagent/PROJECT.md` for the full picture and `.jagent/planning/` for the
execution board (`STATE.md` → `TASKS.md` → `tickets/`).

Depends on crates.io `crush-pkg` / `crush-lang-sdk` 0.10 (no path-dep); to
develop against crush-ast sources use a gitignored `.cargo/config.toml`
`[patch.crates-io]` for BOTH crates (README) and never commit the Cargo.lock
it produces. Published to crates.io as
`crush-squeeze` (the binary is `squeeze`); see `RELEASE.md`. Lives in
`projects/crush-workspace/` alongside crush-ast's other consumer repos (see
`../CLAUDE.md` and `../README.md` for that grouping). Never commit to
`master` directly on shared checkouts; this repo has none of that
shared-checkout history yet, but the convention still applies going forward.

Remote: `nixpt/squeeze` (public, pushed 2026-07-14).

## Build

```bash
cargo build   # binary: target/debug/squeeze
cargo test    # tests/cli.rs: the real binary over temp dirs, no network
```

## Project memory

This repo uses [dejavue](https://github.com/nixpt/dejavue) for persistent architectural context.
Run `dejavue context` before making changes.
Fallback if not on PATH: `python3 .dejavue/dejavue context`

<!-- dejavue:discovery -->
