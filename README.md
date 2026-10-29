# cfops CLI tool

CLI tooling for Cloudflare operations, distributed via [mise](https://mise.jdx.dev/).

## Overview

`cfops` is a credential-injecting wrapper around Wrangler and Terraform for managing Cloudflare Workers infrastructure.

- Distributed as a [mise](https://mise.jdx.dev/) tool: projects declare it as a dependency, and mise activates it on `$PATH` only when inside that project directory.
- Project-specific Cloudflare details are maintained in the project's `.envrc`.

## Using cfops

A Cloudflare account's API token and account ID are stored as secrets in Bitwarden Secrets Manager. The machine account token that grants access to those secrets is stored in macOS Keychain — nothing is on disk or in the shell environment until it's needed.

At the project level, two files wire it together:

- `.envrc` — declares `BWS_PROFILE_NAME`, pointing at the Bitwarden machine account for the target Cloudflare account (staging or prod)
- `.mise.toml` — declares `cfops` and its dependencies as project tools, activated automatically on `cd`

When you run `cfops -wr deploy` or `cfops -tf apply`, cfops loads the Bitwarden token from Keychain, fetches the Cloudflare credentials, injects them for that command only, and delegates to Wrangler or Terraform. Nothing persists to the shell.

## Getting Started

- [Setup](docs/setup.md)
- [Usage](docs/usage.md)
- [Pipeline](docs/pipeline.md)

## Prerequisites

| | |
|---|---|
| [Cloudflare](https://dash.cloudflare.com) | Account with API token and account ID |
| [Bitwarden Secrets Manager](https://bitwarden.com/products/secrets-manager/) | Account with secrets and machine account |
| [macOS Keychain](https://support.apple.com/en-au/guide/keychain-access/welcome/mac) | Stores the Bitwarden machine account token |
| [mise](https://mise.jdx.dev/) | Installs and activates cfops and its dependencies per-project |

## Dependencies

| Tool | Purpose | mise tool |
|------|---------|---------|
| [direnv](https://direnv.net/) | Project-scoped env vars | `direnv` |
| [bws](https://bitwarden.com/help/secrets-manager-cli/) | Bitwarden Secrets Manager CLI | `bitwarden-secrets-manager` |
| [Wrangler](https://developers.cloudflare.com/workers/wrangler/) | Cloudflare Workers CLI | `npm:wrangler` |
| [Terraform](https://developer.hashicorp.com/terraform) | Infrastructure provisioning | `terraform` |

> All dependencies are managed via `.mise.toml` — see [Setup](docs/setup.md).
