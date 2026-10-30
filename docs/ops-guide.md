# Cloudflare Operations Guide

Covers tooling, provisioning, and deployment for this project. For platform and application structure see [architecture.md](architecture.md).

- [1. Tooling](#1-tooling)
- [2. Environments](#2-environments)
  - [2.1 Environment Strategy](#21-environment-strategy)
  - [2.2 Environment Governance](#22-environment-governance)
- [3. Provisioning](#3-provisioning)
  - [3.1 Conventions](#31-conventions)
  - [3.2 Deploy](#32-deploy)
  - [3.3 Deployment Status](#33-deployment-status)
  - [3.4 Delete](#34-delete)
  - [3.5 Terraform *(future)*](#35-terraform-future)

## 1. Tooling

[cfops](https://github.com/ctracey/cfops) is the ops tool for managing Cloudflare deployments in this project. It wraps Wrangler and Terraform with credential injection via Bitwarden Secrets Manager, so credentials are never stored on disk or exported to the shell.

See cfops [setup](https://github.com/ctracey/cfops/blob/main/docs/setup.md) and [usage](https://github.com/ctracey/cfops/blob/main/docs/usage.md) for installation and credential configuration.

## 2. Environments

### 2.1 Environment Strategy

Two environments: staging for testing, prod for the live instance. Each environment is a separate Cloudflare account with its own `workers.dev` subdomain and API token.

| Environment | Purpose | URL |
|-------------|---------|-----|
| Staging | Testing and validation | `<worker-name>.<staging-subdomain>.workers.dev` |
| Prod | Live instance | Custom domain *(future)* / `<worker-name>.<prod-subdomain>.workers.dev` |

The account is the environment boundary — `wrangler.toml` is identical for both, the active environment is determined solely by which credentials are in use. Developers hold only staging credentials; prod credentials live only in the pipeline.

Switching environments locally is done by updating `BWS_PROFILE_NAME` in `.envrc` to point at the Bitwarden machine account for the target environment. See [cfops usage § Switching Environments](https://github.com/ctracey/cfops/blob/main/docs/usage.md#3-switching-environments).

### 2.2 Environment Governance

Production deployments must go through an approved pipeline or be performed by a user explicitly acting in a prod-admin role. No developer should be able to deploy to prod incidentally or as a side effect of normal workflow.

| Environment | Deploy access |
|-------------|---------------|
| Staging | Any developer, manual or pipeline |
| Prod | Pipeline only, or explicit prod-admin action |

This is a principle — enforcement mechanisms (scoped API tokens, pipeline gates, access policies) will be layered in as the project matures.

## 3. Provisioning

### 3.1 Conventions

Worker names reflect type only — environment separation is handled by the account, not the name.

| Suffix | Use |
|--------|-----|
| `<name>-static` | Serves static assets only, no compute logic |
| `<name>-host` | Compute worker with application logic |

### 3.2 Deploy

Wrangler creates the worker on first deploy and serves assets from `dist/`. No pre-provisioning required for `*.workers.dev`:

```bash
cfops -wr deploy
```

On first deploy Cloudflare prompts for a `workers.dev` subdomain — this is account-level and applies to all workers. The deployed URL will be:

```
https://<worker-name>.<your-subdomain>.workers.dev
```

To see all current deployments for the active account, navigate to **Workers & Pages** in the Cloudflare dashboard.

> TODO: add `cfops -ls` command to open the Workers & Pages dashboard for the active account directly from the CLI.

### 3.3 Deployment Status

Check the current deployment status (uses name from `wrangler.toml`):

```bash
cfops -wr deployments status
```

### 3.4 Delete

Delete the worker (uses name from `wrangler.toml`). Prompts for confirmation:

```bash
cfops -wr delete
```

### 3.5 Terraform *(future)*

Terraform will be used when a custom domain is added. Resources needed:

- `cloudflare_workers_script` — declares the worker and its assets binding
- `cloudflare_workers_domain` — attaches a custom domain

```bash
cfops -tf init
cfops -tf plan
cfops -tf apply
```
