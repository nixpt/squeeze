# squeeze

Intuitive one-command build tool for the Crush ecosystem.

## Identity

- **Repository:** squeeze
- **Language:** Rust (edition 2024)
- **Ecosystem:** Peer of `crush-ast` (path-dep on `../crush-ast/crates/crush-pkg` +
  `../crush-ast/crates/crush-vm`, flat `projects/` layout — same pattern `buckets`
  uses to reach into other peers). Not a fork or replacement of `crush-pkg`: it's a
  thin composition layer over `crush-pkg`'s existing `PackageBuilder` pipeline.
- **Protocol:** CLI binary (`squeeze`). No library API of its own yet.

## Why this exists

`crush-pkg build` and `crush-pkg run` are two separate, uncomposed steps: `run`
interprets the manifest's entry source directly (via `PayloadFormat`/`CapsuleRunner`
detection) — it never calls `build()`, never needs `target/` to exist, and never
writes build artifacts. There's no single command that does check → build →
run the way `cargo run` does. `squeeze` (no subcommand) is exactly that: one
command, one status line per stage, sensible defaults — while `squeeze
build`/`check`/`run` stay available individually.

## Workspace (1 crate)

```
squeeze/
├── src/
│   └── main.rs       # CLI: new / check / build / run + default composed flow
├── Cargo.toml
└── .jagent/          # this planning board
```

## Scope (v0.1)

`language = "crush"` capsules only, via `crush_pkg::builder::PackageBuilder` +
`crush_vm::run_with_caps` directly. `crush-pkg`'s own CLI `run` command also
supports Script (bun/node/deno/python) and Native capsules through the
`CapsuleRunner`/`CrushRunner` trait system in `crush_pkg::runners` — squeeze
doesn't wire that in yet (see `SQUEEZE-1`). A non-`crush` `language` value in
the manifest errors out explicitly rather than being silently mis-handled.

## Naming

Named after Crush's own fruit-crushing metaphor (crush the grapes → squeeze
the juice out → that's the build). Considered and rejected "bob"/"bob the
builder" — heavily conflicted (`BobBuildTool/bob`, `benchkram/bob`,
`bobbin-cli`, a published `bob` crate on crates.io). See dejavue decision
"squeeze: new peer repo wrapping crush-pkg..." for the full naming record.
