# Releasing squeeze

squeeze is published to crates.io as **`crush-squeeze`** (the name `squeeze`
belongs to an unrelated 2020 compression library); the binary it installs is
`squeeze`. It depends only on published crates (`crush-pkg` from crush-ast's
0.8.x line), so it publishes from any checkout.

## Steps

1. On `master`, move the `[Unreleased]` section of `CHANGELOG.md` under the new
   version, and set `version` in `Cargo.toml` (SemVer per
   `SQUEEZE-6-v1-cli-stability-contract.md`).
2. Check, from a clean tree:

   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo package            # builds the crate from its tarball
   ```

3. Live check: `squeeze new demo && cd demo && squeeze` (stdin from
   `/dev/null` if the program reads input) prints the program's output.
4. `cargo publish`, then tag the commit `vX.Y.Z` and push the tag.

## History

0.1.0 (2026-07-16) was prepared but never published: crush-pkg and crush-vm
0.3.0 weren't on crates.io yet, and the name `squeeze` turned out to be taken.
The first published release is `crush-squeeze` 0.1.0, on crush-pkg 0.8.0.
