# Usage Guide

Operational commands for worker provisioning and DNS config. Assumes [setup](setup.md) is complete.

- [1. Operations](#1-operations)
- [2. Switching Environments](#2-switching-environments)
- [3. Setting Up Additional Projects](#3-setting-up-additional-projects)

## 1. Operations

Following the installation and setup guides you are ready for operations.

Follow the [ops guide](ops-guide.md) for practical examples for worker provisioning and DNS config (nameserver delegation & hostname routing).

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
cfops -tf init -backend-config=terraform/backend.hcl
cfops -tf plan
cfops -tf apply
cfops -tf -h                      # Full Terraform help — cfops is a full delegation
```

## 2. Switching Environments

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

## 3. Setting Up Additional Projects

Once credentials are in Bitwarden and Keychain for an account, additional projects on that account only need:

```bash
[ -f .mise.toml ] && echo -e "\033[34m→\033[0m ACTION: .mise.toml already exists, manually update .mise config" || curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/mise/.mise.toml
mise install
cfops init
direnv allow
cfops -wr whoami
```
