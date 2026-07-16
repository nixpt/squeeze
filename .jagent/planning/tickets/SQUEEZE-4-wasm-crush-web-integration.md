# SQUEEZE-4 — Verify and document `.cvm` → `crush-web` round-trip (M3)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-4 |
| **Priority** | P2 |
| **Status** | Backlog |
| **Phase** | M3 |
| **Assignee** | unassigned |
| **Dependencies** | SQUEEZE-1 (M2 dispatch stable), SQUEEZE-3 (crates.io crate so deps are inspectable) |
| **Estimated effort** | S — expected mostly docs + a verified example, not new squeeze code |

## Problem

`crush-web` exposes `run_blob(bytes)` that consumes a CVM1 bytecode blob. `squeeze build` already writes `target/<name>.cvm` via `PackageBuilder`. **The hypothesis** is that `Program::to_blob()` produces the same format regardless of "target platform" — CVM1 bytecode is platform-invariant — so `squeeze build` can target wasm32 (or any consumer of `.cvm`) without code changes. This needs to be verified, not assumed; if true, M3 is documentation + an example, not new code. If false, M3 introduces a `--target` flag and a divergence in `write_output`.

## Success criteria

- [ ] Verification artifact: a small example repo (or `examples/wasm-hello/` here) that
  - [ ] builds a crush capsule with `squeeze build`
  - [ ] loads the resulting `target/<name>.cvm` into `crush-web`'s `run_blob` (browser-side)
  - [ ] prints the expected hello-world output in the browser console
- [ ] README "Web target" section captures the recipe (capsule → `squeeze build` → upload `.cvm` to a static host → load via 5-line HTML+JS)
- [ ] If verification fails (CVM1 needs a target bit somewhere): ticket pivots to also producing `--target wasm32` plumbing, and a new sub-task is added here

## Technical approach

1. Stand up the example — clone `crush-web`'s test fixture, or write a minimal `index.html` that imports the wasm module and feeds `run_blob` the bytes from `/target/hello.cvm`.
2. Read `crush-web`'s `run_blob` signature + any reservations about which CVM1 versions / capsules it accepts.
3. Run the capsule in the browser, verify console output matches the CLI output.
4. Capture the recipe in README.

## Files to modify

- `README.md` — add "Web target" section
- (Optional) `examples/wasm-hello/` — minimal example

## Non-goals

- `squeeze build --target wasm32` plumbing — land ONLY if verification proves the platform-invariance hypothesis wrong
- Bundler / Vite / webpack scaffolding — that's the example's problem, not `squeeze`'s
- A wasm-only optimization pass — out of scope for v0.x
