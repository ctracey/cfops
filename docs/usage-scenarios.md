# Usage Scenarios

Covers tooling, provisioning, and deployment for this project.

- [1. Prerequisite: Tooling](#1-prerequisite-tooling)
- [2. Provisioning](#2-provisioning)
  - [2.1 Conventions](#21-conventions)
  - [2.2 Deploy](#22-deploy)
  - [2.3 Deployment Status](#23-deployment-status)
  - [2.4 Delete](#24-delete)
- [3. Domain Routing](#3-domain-routing)
  - [3.1 Domain Config (Cloudflare Zone)](#31-domain-config-cloudflare-zone)
  - [3.2 Traffic Routing (Cloudflare Workers)](#32-traffic-routing-cloudflare-workers)

## 1. Prerequisite: Tooling

[cfops](https://github.com/ctracey/cfops) is the ops tool for managing Cloudflare deployments in this project. It wraps Wrangler and Terraform with credential injection via Bitwarden Secrets Manager, so credentials are never stored on disk or exported to the shell.

See cfops [installation](installation.md) and [setup](setup.md) for installation and credential configuration.

## 2. Provisioning

### 2.1 Conventions

Worker names reflect type only — environment separation is handled by the account, not the name.

| Suffix | Use |
|--------|-----|
| `<name>-static` | Serves static assets only, no compute logic |
| `<name>-host` | Compute worker with application logic |

### 2.2 Deploy

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

### 2.3 Deployment Status

Check the current deployment status (uses name from `wrangler.toml`):

```bash
cfops -wr deployments status
```

### 2.4 Delete

Delete the worker (uses name from `wrangler.toml`). Prompts for confirmation:

```bash
cfops delete
```

## 3. Domain Routing

Routing a custom domain to a worker involves two steps: provisioning the Cloudflare zone via Terraform, then attaching the domain to the worker via Wrangler.

> **Prerequisite:** [DNS Setup Guide](dns/dns-setup.md) must be completed before running any Terraform commands.

### 3.1 Domain Config (Cloudflare Zone)

Provisions the Cloudflare zone for `<DOMAIN>`, configures DNS records, and delegates nameservers at the registrar. See [DNS Domain Config](dns/dns-config-zone.md).

### 3.2 Traffic Routing (Cloudflare Workers)

Attaches `<DOMAIN>` to the worker and provisions the SSL certificate. See [Worker DNS Config](dns/dns-config-worker.md).
