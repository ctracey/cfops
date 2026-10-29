# Usage

TODO: mise binary dependency management

TODO: cloudflare_poc secops docs

## 1. How It Works

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

## 2. Commands

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
