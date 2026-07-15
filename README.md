# squeeze

Intuitive one-command build tool for the Crush ecosystem.

## What this is

`crush-pkg build` and `crush-pkg run` are two separate, uncomposed steps —
`run` interprets the manifest's entry source directly, it never calls
`build()` or writes `target/`. `squeeze` composes check → build → write
`target/` → run into one command with one status line per stage, the way
`cargo run` does:

```bash
squeeze new hello-world
cd hello-world
squeeze              # check, build, run — one command
```

`squeeze new` / `check` / `build` / `run` also work standalone, for when you
want just one step.

`squeeze` doesn't reimplement any of `crush-pkg`'s actual pipeline — it's a
thin composition layer over `crush_pkg::builder::PackageBuilder` and
`crush_vm::run_with_caps`. `crush-pkg` remains the source of truth for
manifest parsing (`capsule.toml`), compilation, and bytecode format.

## What this is not (yet)

Only `language = "crush"` capsules. `crush-pkg` itself also supports Script
(bun/node/deno/python) and Native capsules through a separate runner
system — squeeze doesn't wire that in yet (a non-crush `language` value
errors out explicitly rather than being silently mis-handled). See
`.jagent/planning/tickets/SQUEEZE-1-script-native-capsules.md`.

## Building

```bash
cargo build
./target/debug/squeeze new my-app
```

## Provenance

Peer path-dep on `../crush-ast/crates/{crush-pkg,crush-vm}` (flat
`projects/` layout — same pattern `buckets` uses to reach other peers).
Named after Crush's own fruit-crushing metaphor (crush the grapes → squeeze
the juice out → that's the build). See `.jagent/PROJECT.md` for the full
naming story, including why "bob"/"bob the builder" was considered and
rejected (heavily conflicted name — see the dejavue decision log).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE) at your option.
