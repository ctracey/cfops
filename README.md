# cfops

CLI tooling for Cloudflare operations, distributed via [mise](https://mise.jdx.dev/).

## Overview

`cfops` is a credential-injecting wrapper around Wrangler and Terraform for managing Cloudflare Workers infrastructure. Rather than living in `~/.zshrc` or being installed globally, it is distributed as a mise tool: projects declare it as a dependency, and mise activates it on `$PATH` only when inside that project directory.

## 1. Environment Mise Setup

### 1.1 Dependencies

| Tool | Purpose | Install |
|------|---------|---------|
| [mise](https://mise.jdx.dev/) | Per-project tool management | `brew install mise` |

### 1.2 Setup Shell

Add to `~/.zshrc` (once per machine):

```zsh
eval "$(mise activate zsh)"
```

This hooks `cd` so mise can activate per-project tools automatically when you enter a directory.

## 2. Project Setup

### 2.1 Config

Add `cfops` as a mise tool in your project's `.mise.toml`:

```toml
[tools]
"ubi:ctracey/cfops" = "latest"
```

### 2.2 Use

```bash
mise install
```

`cfops` will be available on `$PATH` whenever you're inside that project directory.

## 3. cfops CLI Tool

### 3.1 How It Works

**Dependencies**

| Tool | Purpose | Install |
|------|---------|---------|
| [direnv](https://direnv.net/) | Project-scoped env vars | `brew install direnv` |
| [bws](https://bitwarden.com/help/secrets-manager-cli/) | Bitwarden Secrets Manager CLI | `curl https://bws.bitwarden.com/install \| sh` |
| [Wrangler](https://developers.cloudflare.com/workers/wrangler/) | Cloudflare Workers CLI | `npm install -g wrangler` |
| [Terraform](https://developer.hashicorp.com/terraform) | Infrastructure provisioning | `brew install hashicorp/tap/terraform` |

`cfops` injects Cloudflare credentials from Bitwarden Secrets Manager before delegating to Wrangler or Terraform. Credentials are never stored on disk or exported to the global environment.

- `-wr` — delegates to `wrangler` with credentials injected
- `-tf` — delegates to `terraform` with credentials injected

The active Cloudflare account (staging vs. prod) is controlled by `BWS_PROFILE_NAME` in the project's `.envrc`. Switching environments is done by updating that value — the tooling itself is account-agnostic.

### 3.2 cfops Usage

**Wrangler**

```bash
cfops -wr deploy                  # Deploy the worker
cfops -wr deployments status      # Check deployment status
cfops -wr delete                  # Delete the worker (prompts for confirmation)
```

**Terraform**

```bash
cfops -tf init
cfops -tf plan
cfops -tf apply
```
