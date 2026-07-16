# Handoff

Updated: 2026-07-16T11:00:00-05:00

## Summary

squeeze v0.1 was scaffolded, verified, pushed to nixpt/squeeze (public) on 2026-07-14. On 2026-07-16 a planning refresh landed on feature branch `planning/m1-m6-roadmap-refresh`: ROADMAP now defines M1–M6 with exit criteria, TASKS buckets tasks by milestone, SQUEEZE-1's technical approach updated to **full cutover** for Script/Native, and five new tickets were authored (SQUEEZE-2..6). Merging the planning PR is prerequisite to starting M1 work.

## Next Steps

- Merge `planning/m1-m6-roadmap-refresh` into `master` (user is the merge authority; planning PR carries documentation only, no Rust deltas).
- Start **M1 / SQUEEZE-2**: comprehensive test suite (dev-dep `tempfile`, follow `crush-pkg`'s tempdir + `scaffold_package` pattern). Cover every clap arg, every status-line literal, the documented error paths, declared exit codes.
- Start **M5 / SQUEEZE-3** as M1's second PR: `.github/workflows/ci.yml` running `cargo test` + `cargo deny` on stable + MSRV; `rust-toolchain.toml`; tag-driven release workflow.
- After M1 lands, start **M2 / SQUEEZE-1** (the rewrite that's been waiting): full cutover of `cmd_run` + `cmd_default` through `crush_pkg::runners::handle_run()` for every capsule type. Narrow or remove `require_crush_language`; crush-language `build`/`check` stay on PackageBuilder with an explicit refusal for non-crush.
- After M2 lands, **M3 / SQUEEZE-4**: verify (likely doc-only) that `squeeze build`'s `.cvm` round-trips through `crush-web`'s `run_blob`. Document the recipe.
- After M3 lands, **M4 / SQUEEZE-5**: wrap `crush-pkg`'s `pack`/`sign`/`verify` as `squeeze` subcommands.
- **M6 / SQUEEZE-6** at the end: factor literal status-line strings into `messages.rs`, type the exit codes, capture the surface in `MANUAL.md`, write `CHANGELOG.md` for v0.x → v1.0, set the post-v1.0 breaking-change gate.

## Boot Instructions

Read `.dejavue/handoff.md`, `.dejavue/state.md`, `.dejavue/decisions.md`, and `.dejavue/timeline.jsonl` before making changes. Run `dejavue context` if installed.
