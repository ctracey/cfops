# Pipeline

## Release

Releases are created automatically by GitHub Actions when a version tag is pushed. The workflow attaches `bin/cfops` as a release asset, making it available for download by mise/ubi.

### Cut a release

```bash
git tag v<major>.<minor>.<patch>
git push origin v<major>.<minor>.<patch>
```

The workflow at `.github/workflows/release.yml` triggers on any `v*` tag, creates a GitHub release, and uploads `bin/cfops` as the asset.

### Install in a consuming project

Pin to a specific version in `.mise.toml`:

```toml
[tools]
"ubi:ctracey/cfops" = "0.1.0"
```

Or use `latest`, noting that mise holds new releases for 24 hours before they are eligible:

```toml
[tools]
"ubi:ctracey/cfops" = "latest"
```

To bypass the 24-hour hold (suitable for personal/ops tooling), add to `~/.config/mise/config.toml`:

```toml
[settings]
minimum_release_age = "0"
```

Then run:

```bash
mise install
```
