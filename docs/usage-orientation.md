# Usage Orientation

High-level orientation to cfops — how credential injection works, the environment and account model, and what switching environments looks like in practice. For step-by-step operations, go directly to [Usage Scenarios](usage-scenarios.md).

- [1. Operations](#1-operations)
- [2. Environments](#2-environments)
  - [2.1 Environment Strategy](#21-environment-strategy)
  - [2.2 Environment Governance](#22-environment-governance)
- [3. Managing Context](#3-managing-context)
  - [3.1 Switching Environments](#31-switching-environments)
  - [3.2 Setting Up Additional Projects](#32-setting-up-additional-projects)

## 1. Operations

Following the installation and setup guides you are ready for operations.

Follow the [ops guide](usage-scenarios.md) for practical examples for worker provisioning and DNS config (nameserver delegation & hostname routing).

### 1.1 Worker Operations

`cfops` injects Cloudflare credentials from Bitwarden before delegating to Wrangler. Credentials are never stored on disk or exported to the shell.

```bash
cfops deploy                      # Deploy the worker
cfops delete                      # Delete the worker (prompts for confirmation)
cfops -wr whoami                  # Verify authenticated account
cfops -wr deployments status      # Any other Wrangler command
cfops list                        # Open Workers & Pages dashboard
cfops -wr -h                      # Full Wrangler help — cfops is a full delegation
```

### 1.2 DNS Config

> **Prerequisite:** [DNS Setup Guide](dns/dns-setup.md) must be completed before running any Terraform commands.

`cfops` injects Cloudflare credentials and R2 state backend credentials before delegating to Terraform.

```bash
cd terraform
cfops -tf init -backend-config=./backend.hcl
cfops -tf plan
cfops -tf apply
cfops -tf -h                      # Full Terraform help — cfops is a full delegation
```

## 2. Environments

### 2.1 Environment Strategy

Two environments: staging for testing, prod for the live instance. Each environment is a separate Cloudflare account with its own `workers.dev` subdomain and API token.

| Environment | Purpose | URL |
|-------------|---------|-----|
| Staging | Testing and validation | `<worker-name>.<staging-subdomain>.workers.dev` |
| Prod | Live instance | Custom domain *(future)* / `<worker-name>.<prod-subdomain>.workers.dev` |

The account is the environment boundary — `wrangler.toml` is identical for both, the active environment is determined solely by which credentials are in use. Developers hold only staging credentials; prod credentials live only in the pipeline.

### 2.2 Environment Governance

Production deployments must go through an approved pipeline or be performed by a user explicitly acting in a prod-admin role. No developer should be able to deploy to prod incidentally or as a side effect of normal workflow.

| Environment | Deploy access |
|-------------|---------------|
| Staging | Any developer, manual or pipeline |
| Prod | Pipeline only, or explicit prod-admin action |

This is a principle — enforcement mechanisms (scoped API tokens, pipeline gates, access policies) will be layered in as the project matures.

## 3. Managing Context

### 3.1 Switching Environments

Each environment (staging, prod) is a separate Cloudflare account with its own Bitwarden machine account. The active environment is determined by `BWS_PROFILE_NAME` in `.envrc`.

To switch, update `BWS_PROFILE_NAME` and reload:

```bash
# in .envrc
export BWS_PROFILE_NAME=bws_<target-BWS_MACHINE_ACCOUNT_NAME>
```

```bash
direnv allow
eval "$(direnv export zsh)"
```

`wrangler.toml` does not change — the same config deploys to whichever account the credentials belong to.

### 3.2 Setting Up Additional Projects

Once credentials are in Bitwarden and Keychain for an account, additional projects on that account only need:

```bash
[ -f .mise.toml ] && echo -e "\033[34m→\033[0m ACTION: .mise.toml already exists, manually update .mise config" || curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/mise/.mise.toml
mise install
cfops init
direnv allow
cfops -wr whoami
```
