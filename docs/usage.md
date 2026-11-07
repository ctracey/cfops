# Usage Guide

Operational commands for worker provisioning and DNS config. Assumes [setup](setup.md) is complete.

- [1. Worker Operations](#1-worker-operations)
- [2. DNS Config](#2-dns-config)
- [3. Switching Environments](#3-switching-environments)
- [4. Setting Up Additional Projects](#4-setting-up-additional-projects)

## 1. Worker Operations

`cfops` injects Cloudflare credentials from Bitwarden before delegating to Wrangler. Credentials are never stored on disk or exported to the shell.

```bash
cfops deploy                      # Deploy the worker
cfops delete                      # Delete the worker (prompts for confirmation)
cfops -wr whoami                  # Verify authenticated account
cfops -wr deployments status      # Any other Wrangler command
cfops list                        # Open Workers & Pages dashboard
```

## 2. DNS Config

`cfops` injects Cloudflare credentials and R2 state backend credentials before delegating to Terraform.

```bash
cfops -tf init -backend-config=terraform/backend.hcl
cfops -tf plan
cfops -tf apply
```

For the full DNS provisioning workflow — zone creation, www redirect, nameserver delegation — see [DNS Config](dns/dns.md).

## 3. Switching Environments

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

## 4. Setting Up Additional Projects

Once credentials are in Bitwarden and Keychain for an account, additional projects on that account only need:

```bash
[ -f .mise.toml ] && echo -e "\033[34m→\033[0m ACTION: .mise.toml already exists, manually update .mise config" || curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/mise/.mise.toml
mise install
cfops init
direnv allow
cfops -wr whoami
```
