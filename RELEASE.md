# Releasing squeeze

squeeze is published to crates.io as **`crush-squeeze`** (the name `squeeze`
belongs to an unrelated 2020 compression library); the binary it installs is
`squeeze`. It depends only on published crates (`crush-pkg` and
`crush-lang-sdk` from crush-ast, which move together), so it builds and
publishes from any checkout.

## Steps

1. On a branch, move the `[Unreleased]` section of `CHANGELOG.md` under the
   new version with today's date, and set `version` in `Cargo.toml` (SemVer
   per `SQUEEZE-6-v1-cli-stability-contract.md`: on 0.x a `feat` is a minor
   bump). If crush-ast released, raise `crush-pkg` and `crush-lang-sdk`
   together and run `cargo update -p crush-pkg -p crush-lang-sdk`.
2. Open a PR. CI (`ci.yml`) runs fmt, clippy, the tests, `cargo package` and
   an MSRV check. Merge it when green.
3. Live check from the merged tree: `squeeze new demo && cd demo && squeeze`
   (stdin from `/dev/null` if the program reads input) prints the program's
   output.
4. Tag the merge commit and push the tag:

   ```bash
   git tag -a vX.Y.Z -m "squeeze vX.Y.Z" <merge-commit>
   git push origin vX.Y.Z
   ```

   `release.yml` then publishes `crush-squeeze` X.Y.Z to crates.io (Trusted
   Publishing; a version already there is skipped) and creates the GitHub
   release with `squeeze-X.Y.Z-x86_64-unknown-linux-gnu.tar.gz` and
   `SHA256SUMS`, its notes taken from the CHANGELOG section. Re-run it for
   an existing tag with `gh workflow run release.yml -f tag=vX.Y.Z`.

## Trusted Publishing (one-time, on crates.io)

crush-squeeze → Settings → Trusted Publishing → add: repository owner
`nixpt`, repository `squeeze`, workflow `release.yml`, environment empty.
Check it without publishing: `gh workflow run release.yml -f verify_auth=true`.

## History

0.1.0 (2026-07-16) was prepared but never published: crush-pkg and crush-vm
0.3.0 weren't on crates.io yet, and the name `squeeze` turned out to be taken.
The first published release is `crush-squeeze` 0.1.0 (2026-10-31), on
crush-pkg 0.8.0. 0.2.0 is SQUEEZE-8 (squeeze as Crush's cargo) on crush-pkg
0.10.0, and the first release cut by `release.yml`.
