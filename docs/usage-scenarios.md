# Usage Scenarios

Covers tooling, provisioning, and deployment for this project.

- [1. Prerequisite: Tooling](#1-prerequisite-tooling)
- [2. Worker Config](#2-worker-config)
  - [2.1 Worker Naming Convention](#21-worker-naming-convention)
  - [2.2 Create/Config Worker](#22-createconfig-worker)
- [3. Provisioning Workers](#3-provisioning-workers)
  - [3.1 Deploy](#31-deploy)
  - [3.2 Deployment Status](#32-deployment-status)
  - [3.3 Delete](#33-delete)
- [4. Domain Routing](#4-domain-routing)
  - [4.1 Domain Config (Cloudflare Zone)](#41-domain-config-cloudflare-zone)
  - [4.2 Traffic Routing (Cloudflare Workers)](#42-traffic-routing-cloudflare-workers)

## 1. Prerequisite: Tooling

[cfops](https://github.com/ctracey/cfops) is the ops tool for managing Cloudflare deployments in this project. It wraps Wrangler and Terraform with credential injection via Bitwarden Secrets Manager, so credentials are never stored on disk or exported to the shell.

See cfops [installation](installation.md) and [setup](setup.md) for installation and credential configuration.

## 2. Worker Config

### 2.1 Worker Naming Convention

Worker names reflect type only — environment separation is handled by the account, not the name.

| Suffix | Use |
|--------|-----|
| `<name>-static` | Serves static assets only, no compute logic |
| `<name>-host` | Compute worker with application logic |

### 2.2 Create/Config Worker

Run `cfops init` and answer yes to "Set up a worker?". If `.envrc` already exists it skips straight to the worker prompt:

```bash
cfops init
```

Prompts for:
- Worker name (see [naming convention](#21-worker-naming-convention))
- Assets directory (default: `./dist`)

This writes `wrangler.toml` to the project root using the [static worker template](../templates/wrangler/static-worker/wrangler.toml), with today's date set as the compatibility date.

> TODO:
> - test this end to end
> - check flow and scenario make sense
> - add clear step UX (checklist as user progresses)

## 3. Provisioning Workers

### 3.1 Deploy

Wrangler creates the worker on first deploy and serves assets from `dist/`. No pre-provisioning required for `*.workers.dev`:

```bash
cfops deploy
```

On first deploy Cloudflare prompts for a `workers.dev` subdomain — this is account-level and applies to all workers. The deployed URL will be:

```
https://<worker-name>.<your-subdomain>.workers.dev
```

To see all current deployments for the active account:

```bash
cfops list
```

This opens the Workers & Pages dashboard for the active account directly in the browser.

### 3.2 Deployment Status

Check the current deployment status (uses name from `wrangler.toml`):

```bash
cfops -wr deployments status
```

### 3.3 Delete

> **WARNING:** This will delete the worker from Cloudflare — only run when that is your intention.

Delete the worker (uses name from `wrangler.toml`). Prompts for confirmation:

```bash
cfops delete
```

## 4. Domain Routing

Routing a custom domain to a worker involves two steps: provisioning the Cloudflare zone via Terraform, then attaching the domain to the worker via Wrangler.

> **Prerequisite:** [DNS Setup Guide](dns/dns-setup.md) must be completed before running any Terraform commands.

### 4.1 Domain Config (Cloudflare Zone)

Provisions the Cloudflare zone for `<DOMAIN>`, configures DNS records, and delegates nameservers at the registrar. See [DNS Domain Config](dns/dns-config-zone.md).

### 4.2 Traffic Routing (Cloudflare Workers)

Attaches `<DOMAIN>` to the worker and provisions the SSL certificate. See [Worker DNS Config](dns/dns-config-worker.md).
