# Publishing

A release is one run of **Cut release**, from the Actions tab. There is no local release command.

Cut release commits the new version and pushes a `v<version>` tag, which starts **Publish release** (`.github/workflows/release.yml`). Publish release checks that the tag matches the version in `Cargo.toml`, publishes to crates.io (trusted publishing, no token), then creates the GitHub release. Tags with a prerelease suffix (`v1.3.0-rc.1`) become GitHub prereleases.

## Releasing

**Actions → Cut release → Run workflow**, on `main`:

| Input | Values |
| --- | --- |
| bump | `patch`, `minor` or `major` |
| pre | `none` for a stable release, or `alpha`, `beta`, `rc` |
| version | an exact version instead, such as `1.3.0-rc.1` (overrides the two above) |
| dry run | pick the version and run the tests, push nothing |

It updates `Cargo.toml` and `Cargo.lock`, runs `cargo test --locked`, commits `release: v<version>` to `main`, pushes the tag and starts Publish release. The run summary shows the version it picked.

How the version is picked, from the current one:

| Current | bump | pre | Next |
| --- | --- | --- | --- |
| 1.2.3 | patch | none | 1.2.4 |
| 1.2.3 | minor | none | 1.3.0 |
| 1.2.3 | major | none | 2.0.0 |
| 1.2.3 | minor | beta | 1.3.0-beta.1 |
| 1.3.0-beta.1 | minor | beta | 1.3.0-beta.2 |
| 1.3.0-beta.2 | minor | rc | 1.3.0-rc.1 |
| 1.3.0-rc.1 | minor | none | 1.3.0 |
| 1.3.0-rc.1 | major | none | 2.0.0 |

From a prerelease, a bump that already leads to its version stays on it, so keep the same `bump` through the betas, the release candidates and the release. Going back (from `rc` to `beta`) is refused. To see a pick without releasing, run Cut release with **dry run**, or `python scripts/release.py next-version minor rc`.

## When a release fails

- **Publish release failed on a transient error** (runner or network): open the run and use **Re-run failed jobs**.
- **It needs a fix, and crates.io does not have the version yet**: delete the tag (Code → Tags, or `git push origin --delete v<version>`) and the GitHub release if one was created, merge the fix to `main`, then run Cut release with **version** set to the same version. It tags `main` as it is, since the version is already there.
- **crates.io already has the version**: it cannot be published again. Release the fix as the next patch.

## Automation set up once

### Updating the tools: `DEPENDENTS_TRIGGER_TOKEN`

After a stable release, Publish release starts **Update gdsyntax** in gdhealth, gdtest and gdtidy, which opens or updates a PR there moving them to the new gdsyntax (see their PUBLISHING.md). Starting workflows in other repositories takes a token that can do only that:

1. Account settings → Developer settings → Personal access tokens → **Fine-grained tokens** → Generate new token.
2. Name it `gdsyntax dependents trigger`, owner `q-rault`, with an expiration.
3. Repository access: **Only select repositories** → `q-rault/gdhealth`, `q-rault/gdtest`, `q-rault/gdtidy`.
4. Repository permissions: **Actions: Read and write** (nothing else).
5. Store it in gdsyntax: `gh secret set DEPENDENTS_TRIGGER_TOKEN -R q-rault/gdsyntax`.

Without it, the release still succeeds and warns that the tools were not told; run Update gdsyntax in each of them by hand then. A new tool that uses gdsyntax needs `update-gdsyntax.yml` (copy it from gdhealth) and its name in the `notify-dependents` job of `.github/workflows/release.yml`.
