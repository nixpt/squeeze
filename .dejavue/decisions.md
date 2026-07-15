# Decisions


## 2026-07-14T19:22:53-05:00 — [STRATEGIC] [ADOPTED] [ARCHITECTURAL] squeeze: new peer repo wrapping crush-pkg, named after crush's juice/pressing metaphor

Reason:
crush-pkg build and crush-pkg run are two separate, uncomposed steps today — run interprets the entry source directly via CapsuleRunner/PayloadFormat detection, it never calls build() or writes target/. squeeze's whole value in v0.1 is composing check->build->write target/->run into one command with one status line per stage, the way 'cargo run' does, without reimplementing any of crush-pkg's actual compile/run pipeline (PackageBuilder is the real work; squeeze is a thin composition layer).

Artifacts: src/main.rs, Cargo.toml

Author type: agent

Rejected alternatives:
- **bob / bob-the-builder**: heavily conflicted name — BobBuildTool/bob (embedded/bitbake-style), benchkram/bob (multi-language monorepo build tool, conceptually near-identical to this), bobbin-cli (embedded Rust deploy), a published 'bob' crate on crates.io. Confirmed via web search before naming.
- **crates/squeeze inside crush-ast's own workspace**: would inherit the same feature-unification headaches CRUSHAST-WEB-1 (crush-web) just spent real effort untangling, and squeeze is meant to grow into an ecosystem-wide tool (potentially touching buckets/crush-web/exo-light later), not something crush-ast-workspace-scoped. Standalone peer repo matches the established pattern for new ecosystem tools this session (buckets, interactd, crush-vscode).

Outcome:
v0.1 skeleton: squeeze (default: check+build+run in one flow) / new / check / build / run subcommands, thin wrapper over crush_pkg::builder::PackageBuilder + crush_vm::run_with_caps. Verified end-to-end against a real 'squeeze new demo' -> target/demo.cvm + demo.casm.json -> 'hello from Crush' output, both via the default flow and via 'squeeze run' with no prior build.

