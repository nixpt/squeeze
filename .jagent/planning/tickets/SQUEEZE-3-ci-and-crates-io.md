# SQUEEZE-3 — CI pipeline + crates.io publishing groundwork (M5)

| Field | Value |
|-------|-------|
| **ID** | SQUEEZE-3 |
| **Priority** | P3 |
| **Status** | Backlog |
| **Phase** | M5 |
| **Assignee** | unassigned |
| **Dependencies** | SQUEEZE-2 (test suite for CI to run; lands together with SQUEEZE-2 as M1's second PR) |
| **Estimated effort** | M |

## Problem

`nixpt/squeeze` is a public repo but has no CI — the M1 test suite (SQUEEZE-2) has no automated runner. There's no publishing pipeline to crates.io either — the repo is `nixpt/squeeze`, the crate isn't on the registry yet. Without CI + publishing, downstream consumers can't `cargo install squeeze` and can't depend on the test suite as a regression net for cross-repo PRs.

## Success criteria

- [ ] `.github/workflows/ci.yml` runs:
  - [ ] `cargo test` on the stable Rust toolchain
  - [ ] `cargo check` on the MSRV (e.g. 1.85, pinned in `rust-toolchain.toml`)
  - [ ] `cargo fmt --check` and `cargo clippy -- -D warnings` (NOT in the first iteration — master’s `src/main.rs` is neither fmt-clean nor clippy-clean as of v0.1.0; if added later, capture a one-PR remediation for the existing source first so the CI step has a stable target.)
  - [ ] `cargo deny check advisories` (staged audit of the path-dep tree)
- [ ] `cargo publish --dry-run` succeeds locally — license files, README rendering, `Cargo.toml` metadata (keywords, categories, description length) line up with what crates.io expects
- [ ] Tag-driven release workflow (`.github/workflows/release.yml`) that runs `cargo publish` on `v*` tags — needs `CARGO_REGISTRY_TOKEN` or OIDC trust configured on the repo
- [ ] First release tagged `v0.1.0` post-merge as a smoke test (no behavior change)

## Technical approach

1. Pick a boring Rust GitHub Actions template (likely `dtolnay/rust-toolchain` + `Swatinem/rust-cache`). No exotic actions.
2. Add `rust-toolchain.toml` pinning the MSRV; mirror `crush-ast`'s choice if it has one, otherwise follow `Cargo.toml`'s `rust-version` once we add it.
3. Release workflow uses OpenID Connect (`crates-io-auth-action`) rather than long-lived `CARGO_REGISTRY_TOKEN` if possible.
4. Dry-run `cargo publish` locally and verify README rendering on the temporary index.
5. Tag `v0.1.0` post-merge as the first published release — the planning/version baseline, not a feature marker.

## Files to add

- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `rust-toolchain.toml`
- `(new) deny.toml` — required for `cargo deny check advisories` to be more than a no-op. Configure skip tree, advisories DB source, and bans. (Most existing `crates/-` advisories will be inherited from the config; the file is created from scratch on this branch.)
- (Optional) `.github/dependabot.yml`

## Files to modify

- `Cargo.toml` — add `rust-version` (e.g. `"1.85"`) so crates.io can filter

## Non-goals

- Cross-platform matrix (Windows/macOS/Linux) — out of scope for first iteration; Linux-only CI is fine pre-1.0. Re-evaluate at M6.
- Signed commits / signed tags — follow whatever `crush-ast` does
- `crates-io-owners` / org-level publishing — use the existing `nixpt` singleton account
