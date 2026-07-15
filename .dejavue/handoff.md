# Handoff

Updated: 2026-07-14T19:23:09-05:00

## Summary
squeeze v0.1 scaffolded and verified: check/build/run/new subcommands + default one-shot flow, all thin wrappers over crush_pkg::builder::PackageBuilder. Only language="crush" capsules are wired; Script/Native intentionally out of scope for v0.1 (see main.rs module doc).

## Next Steps
- Add tests (crush-pkg's own test suite is the model: tempdir + scaffold_package + assert on target/ output)
- Wire Script/Native capsule support via crush_pkg::runners::{CapsuleRunner, CrushRunner} instead of the PackageBuilder-only path
- Decide on real GitHub remote (nixpt/squeeze) and push, or fold into an existing org — repo is currently local-only
- Consider: crush-web's new run_blob(bytes) could let 'squeeze build' target wasm32 output too — natural next integration point between the two things built this session

## Boot Instructions
Read `.dejavue/handoff.md`, `.dejavue/state.md`, `.dejavue/decisions.md`, and `.dejavue/timeline.jsonl` before making changes.
