# TASKS — squeeze

## P0 — Skeleton ✅

- [x] `squeeze new <name>` scaffolds via `crush_pkg::manifest::scaffold_package`
- [x] `squeeze check` / `squeeze build` / `squeeze run` — thin wrappers over `PackageBuilder`
- [x] Default (no subcommand): check → build → write `target/` → run, one status line per stage
- [x] Non-`crush` `language` value errors explicitly (`require_crush_language`), not silently mis-handled
- [x] Verified end-to-end: real `squeeze new demo` → `target/demo.cvm` + `.casm.json` → `hello from Crush`

## P1 — Test coverage

- [ ] Test harness following `crush-pkg`'s own pattern (`tempfile::tempdir()` + `scaffold_package` + assert on `target/` contents)
- [ ] Cover: `new`, `check`, `build`, `run`, default flow
- [ ] Cover: "no capsule.toml in cwd" error path
- [ ] Cover: "non-crush language" error path (`require_crush_language`)

## P2 — Script/Native capsule support

- [ ] See `tickets/SQUEEZE-1-script-native-capsules.md`

## P3 — Wasm target (crush-web integration)

- [ ] Scope: does `squeeze build` need a `--target` flag, or does `crush-web`'s
      `run_blob` just consume whatever `.cvm` `squeeze build` already writes?
      (Likely the latter — `Program::to_blob()`'s format doesn't encode a
      target platform, it's the same CVM1 bytecode either way.)
- [ ] If the above holds, this milestone may just be documentation + an
      example, not new squeeze code.

## P4 — Distribution

- [ ] `squeeze` wrapper for `crush-pkg`'s `pack`/`sign`/`verify`/`site`/`site-extract`
