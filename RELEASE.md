# Release process — `squeeze` v0.1.0 (smoke publish)

This document is the publish procedure for the FIRST crates.io release of
`squeeze`. It captures the current state, the upstream dependency that gates
the actual publish, and how to fire the publish once the gate is open.

## Status

| Field | Value |
|-------|-------|
| Branch | `release/v0.1.0` |
| Worktree | `../squeeze--release-v0.1-0` |
| Base | `master` @ `63cb23d` |
| Tag | not yet — `v0.1.0` lands post-publish |
| crates.io | blocked on upstream `crush-pkg v0.3.0` / `crush-vm v0.3.0` |

## Upstream gate (the only blocker)

`cargo publish --dry-run` from `master` and from this branch returns:

```
error: failed to prepare local package for uploading
Caused by:
  no matching package named `crush-pkg` found
  location searched: crates.io index
  required by package `squeeze v0.1.0`
```

…because `crates.io` doesn't yet host `crush-pkg` or `crush-vm` at version
`0.3.0`. `cargo publish` cannot resolve path-deps to siblings at packaging
time when those siblings aren't on the registry. **Until upstream publish
lands, `squeeze v0.1.0` cannot be published to crates.io.** No workaround in
`squeeze` itself can fix this — the fix is upstream.

Recommended upstream sequence (run from the `crush-ast` workspace, NOT from
`squeeze`):

```bash
# 1) From the crush-ast checkout, prepare and verify each top-level crate for publish
#    (mirror whatever pre-publish steps the crush-ast workspace already uses).
# 2) Publish `crush-pkg v0.3.0` (no `--tag` here — tag the commit separately, see below):
cargo publish -p crush-pkg
# 3) Publish `crush-vm v0.3.0`:
cargo publish -p crush-vm
```

After both are on crates.io, come back to this branch and run the squeeze
publish (next section).

## What landed on this branch (vs `master`)

Three source-level changes — no Rust code touched:

1. **`Cargo.toml` — `[package]`**:
   - Added `rust-version = "1.85"` (mirrors the `edition = "2024"` MSRV).
   - Added `exclude = [".dejavue/**", ".jagent/**"]` so the published tarball
     does NOT include the agent-memory and planning board directories. Without
     this, every consumer would download our journaling artifacts.
2. **`Cargo.toml` — `[dependencies]`**:
   - Changed `crush-pkg` and `crush-vm` from
     `{ path = "...",  version = "0.3.0" }` to `version = "0.3.0"` only.
     Path-deps with a `version` requirement trigger a registry index lookup at
     `cargo publish` time; without the `version`, neither dep resolves via
     registry lookups at all.
3. **(removed) `[patch.crates-io]`**:
   - The earlier draft of this branch added a `[patch.crates-io]` redirect
     to compensate for the missing path-deps. It has been REMOVED: a
     committed `[patch.crates-io]` pointing at `../../crush-ast/crates/...`
     would block any non-author `git clone nixpt/squeeze && cargo build`
     because that path doesn’t exist outside the sibling-workspace checkout.
     `cargo install squeeze` from registry works either way; the patch
     served no end-user purpose, only local-dev ergonomics, and is replaced
     here by **a clean registry-pure Cargo.toml**. The squeeze maintainer's
     local dev for the developer who needs the sibling workspace present
     continues to work via original path-deps (kept on `master`); if a
     squash-side developer needs it on this release branch, opt in via a
     personal `[patch.""` override in their local config, not in the
     committed Cargo.toml.

A new `CHANGELOG.md` documents the v0.1.0 entry.

## Pre-flight (run from this worktree, BEFORE the actual publish)

```bash
cd /workspace/projects/crush-workspace/squeeze--release-v0.1-0

# 1) Local build — must succeed via [patch.crates-io]
cargo check

# 2) Tarball preview — confirms what's IN the package
cargo package --list
# Expected top-level:  Cargo.toml, Cargo.lock, README.md, LICENSE-APACHE,
#                      LICENSE-MIT, src/, .gitignore (maybe), and a few auto-generated files.
# Should NOT contain: .dejavue/, .jagent/, target/, samples/, examples/.

# 3) Full packaging dry-run — will succeed when upstream crush-pkg / crush-vm
#    are on crates.io; today it returns the upstream-blocker error.
cargo publish --dry-run --allow-dirty
```

If `cargo publish --dry-run` returns the upstream-blocker error, do not
publish — wait for upstream `crush-pkg v0.3.0` and `crush-vm v0.3.0`.

## Authentication: pick one

For the actual `cargo publish` invocation in step 4 of "Publishing squeeze itself", pick one:

- **Token-based (simple, local):** `cargo login` once against the `nixpt`
  crates.io account; cargo stores a token in `~/.cargo/credentials.toml`.
  Then `cargo publish` uses that token. Long-lived secret — fine for the
  one-time v0.1.0 smoke publish.
- **OIDC Trusted Publishing (recommended for repeat releases):** crates.io
  has shipped OIDC-via-GitHub-Actions since 2025. Configure a trusted
  publisher on crates.io pointing at the `nixpt/squeeze` GitHub repo /
  workflow; no long-lived secret in CI. See
  `https://crates.io/docs/trusted-publishing` and the
  `dtolnay/cargo-workspaces` / `softprops/action-gh-release` ecosystem.

Either path is fine; OIDC is what we'll want post-M1 when CI is live.

## Publishing squeeze itself

Once upstream `crush-pkg v0.3.0` and `crush-vm v0.3.0` are live on crates.io:

```bash
# 1) Merge release/v0.1.0 -> master (or keep separate for tag purposes)
cd /workspace/projects/crush-workspace/squeeze
git checkout master
git merge --no-ff release/v0.1.0 -m 'release: prep for squeeze v0.1.0 (smoke publish)'

# 2) Tag v0.1.0 on master
git tag -a v0.1.0 -m 'squeeze v0.1.0 \u2014 first crates.io release (smoke publish)'

# 3) Push the tag and the merge
git push origin master
git push origin v0.1.0

# 4) Publish (requires CARGO_REGISTRY_TOKEN environment variable OR
#    `cargo login` against the nixpt crates.io account \u2014 the user owns this).
cd /workspace/projects/crush-workspace/squeeze--release-v0.1-0
cargo publish              # NOT --dry-run; this is the real publish.

# 5) Verify on crates.io
curl -sS https://crates.io/api/v1/crates/squeeze | jq '.crate | {name, max_version, downloads, recent_downloads}'
```

## Post-publish checklist

- [ ] `cargo install squeeze` works on a fresh machine (verify the install resolves `crush-pkg` and `crush-vm` from the registered `0.3.0`).
- [ ] The crates.io page renders README correctly (logs / metadata capture).
- [ ] docs.rs builds `squeeze @ v0.1.0` cleanly.
- [ ] `nixpt/squeeze` GitHub Releases page gets a matching `v0.1.0` tag pointing at the merge commit.

## Rollback / yank

crates.io publishes are effectively immutable — there is no `cargo unpublish`. A
yanked release is the only reversal path and is a public signal: avoid yanking
v0.1.0 unless a critical security issue is found. Routine fixups go via
`v0.1.1`.

## Why this is a "smoke" publish, not v1.0

`v0.1.0` ships > what already ships, but without:

- M1 tests / CI (SQUEEZE-2)
- M2 Script/Native support via the runner dispatch (SQUEEZE-1)
- M6 v1.0 CLI stability contract ratification (SQUEEZE-6)

Each subsequent milestone bumps `MINOR`/`PATCH` per the contracts captured in
`SQUEEZE-6`. The goal of publishing `v0.1.0` pre-M1 is purely to unblock
downstream consumer peers (`crush-web`, `crush-notebook`) that want to
`cargo install squeeze` from crates.io rather than clone-and-build.
