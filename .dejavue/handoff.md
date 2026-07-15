# Handoff

Updated: 2026-07-14T19:57:13-05:00

## Summary
squeeze v0.1 scaffolded, verified, moved into projects/crush-workspace/, and pushed to nixpt/squeeze (public). check/build/run/new subcommands + default one-shot flow, all thin wrappers over crush_pkg::builder::PackageBuilder. Only language="crush" capsules are wired; Script/Native intentionally out of scope for v0.1 (see main.rs module doc).

## Next Steps
- Add tests (crush-pkg's own test suite is the model: tempdir + scaffold_package + assert on target/ output)
- Wire Script/Native capsule support via crush_pkg::runners::{CapsuleRunner, CrushRunner} instead of the PackageBuilder-only path (SQUEEZE-1)
- Consider: crush-web's run_blob(bytes) could let 'squeeze build' target wasm32 output too — natural next integration point
- Not yet published to crates.io — repo (nixpt/squeeze) is live, crate registry is not

## Boot Instructions
Read `.dejavue/handoff.md`, `.dejavue/state.md`, `.dejavue/decisions.md`, and `.dejavue/timeline.jsonl` before making changes.
