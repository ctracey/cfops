# cfops

CLI tooling for Cloudflare operations, distributed via [mise](https://mise.jdx.dev/).

## Overview

`cfops` provides the `cf` command — a credential-injecting wrapper around Wrangler and Terraform for managing Cloudflare Workers infrastructure. Rather than living in `~/.zshrc` or being installed globally, it is distributed as a mise tool: projects declare it as a dependency, and mise activates it on `$PATH` only when inside that project directory.

## Installation

### Prerequisites

| Tool | Purpose | Install |
|------|---------|---------|
| [mise](https://mise.jdx.dev/) | Per-project tool management | `brew install mise` |
| [direnv](https://direnv.net/) | Project-scoped env vars | `brew install direnv` |
| [bws](https://bitwarden.com/help/secrets-manager-cli/) | Bitwarden Secrets Manager CLI | `curl https://bws.bitwarden.com/install \| sh` |
| [Wrangler](https://developers.cloudflare.com/workers/wrangler/) | Cloudflare Workers CLI | `npm install -g wrangler` |
| [Terraform](https://developer.hashicorp.com/terraform) | Infrastructure provisioning | `brew install hashicorp/tap/terraform` |

### Bootstrap your shell (once)

Add to `~/.zshrc`:

```zsh
eval "$(mise activate zsh)"
eval "$(direnv hook zsh)"
```

### Use in a project

Add `cfops` as a mise tool in your project's `.mise.toml`:

```toml
[tools]
"ubi:ctracey/cfops" = "latest"
```

Then run:

```bash
mise install
```

`cf` will be available on `$PATH` whenever you're inside that project directory.

## Usage

### Wrangler

```bash
cf -wr deploy                  # Deploy the worker
cf -wr deployments status      # Check deployment status
cf -wr delete                  # Delete the worker (prompts for confirmation)
```

### Terraform

```bash
cf -tf init
cf -tf plan
cf -tf apply
```

## How it works

`cf` injects Cloudflare credentials from Bitwarden Secrets Manager before delegating to Wrangler or Terraform. Credentials are never stored on disk or exported to the global environment.

- `-wr` — delegates to `wrangler` with credentials injected
- `-tf` — delegates to `terraform` with credentials injected

The active Cloudflare account (staging vs. prod) is controlled by `BWS_PROFILE_NAME` in the project's `.envrc`. Switching environments is done by updating that value — the tooling itself is account-agnostic.

## Environment model

Two environments — staging and prod — separated by Cloudflare account, not by wrangler configuration:

| Environment | Deploy access |
|-------------|---------------|
| Staging | Any developer, manual or pipeline |
| Prod | Pipeline only, or explicit prod-admin action |

Developers hold staging credentials only. Prod credentials live exclusively in CI.

## Development

```bash
git clone https://github.com/ctracey/cfops
cd cfops
mise install
```

Scripts live in `bin/`. The entry point is `bin/cf`.
