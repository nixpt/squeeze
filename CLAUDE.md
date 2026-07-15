# squeeze

Intuitive one-command build tool for the Crush ecosystem — wraps
`crush-pkg`'s `PackageBuilder` pipeline instead of reimplementing it. See
`.jagent/PROJECT.md` for the full picture and `.jagent/planning/` for the
execution board (`STATE.md` → `TASKS.md` → `tickets/`).

Peer path-dep on `../../crush-ast/crates/{crush-pkg,crush-vm}` — lives in
`projects/crush-workspace/` alongside crush-ast's other consumer repos (see
`../CLAUDE.md` and `../README.md` for that grouping). Never commit to
`master` directly on shared checkouts; this repo has none of that
shared-checkout history yet, but the convention still applies going forward.

Remote: `nixpt/squeeze` (public, pushed 2026-07-14).

## Build

```bash
cargo build   # binary: target/debug/squeeze
cargo check
```

## Project memory

This repo uses [dejavue](https://github.com/nixpt/dejavue) for persistent architectural context.
Run `dejavue context` before making changes.
Fallback if not on PATH: `python3 .dejavue/dejavue context`

<!-- dejavue:discovery -->
