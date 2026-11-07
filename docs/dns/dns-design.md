# DNS Design

Design decisions and architectural choices underpinning the DNS setup for your domain.

- [1. Decisions](#1-decisions)
- [2. Capabilities Overview](#2-capabilities-overview)
- [3. Ownership Model](#3-ownership-model)

## 1. Decisions

Agree these before provisioning — several are difficult to change once infrastructure is in place.

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Environment isolation | Separate Cloudflare accounts per environment | Staging runs in its own Cloudflare account with its own prod-like staging domain; this DNS config applies to the prod account only |
| Canonical address | `<DOMAIN>` (bare domain) | Cloudflare CNAME flattening removes the apex CNAME limitation; cleaner URLs |
| Secondary address | `www.<DOMAIN>` → 301 to `<DOMAIN>` | Cloudflare Redirect Rule in Terraform; no Worker code needed |
| Terraform state backend | Cloudflare R2 | R2 is coming for the application anyway; credentials are already in the toolchain; avoids a local-to-remote migration later |
| Asset naming | Verbose, no abbreviations | e.g. `<RESOURCE_LABEL>-terraform-state` not `<RESOURCE_LABEL>-tf-state`; applies to all Cloudflare asset names |

## 2. Capabilities Overview

Four distinct capabilities govern how credentials are scoped. Each capability should map to a credential authorised for exactly that purpose.

| Capability | Description | Token | Secret |
|------------|-------------|-------|--------|
| Infrastructure state management | Terraform state backend — tracks all Cloudflare resources managed by Terraform | `R2 Storage (<RESOURCE_LABEL>-terraform-state)` | `CLOUDFLARE_R2_TERRAFORM_STATE_*` |
| Worker management | Deploy, delete, and manage workers | `Worker Provision (all)` | `CLOUDFLARE_WORKER_PROVISION_TOKEN` |
| Domain management | Zone, DNS records, redirect rules | `DNS Config (<DOMAIN>)` | `CLOUDFLARE_DNS_CONFIG_TOKEN` |
| Application storage | Worker runtime data in R2 — per-worker buckets | R2 Storage token (future, per-worker) | TBD |

> **Note:** Infrastructure state management and application storage both use R2 User Tokens but are distinct credentials. Application storage tokens are per-worker and will be created when needed.

## 3. Ownership Model

Separation of concerns — each resource is owned by exactly one tool. Terraform owns infrastructure, Wrangler owns application concerns. Do not duplicate ownership.

| Resource | Tool | Why |
|----------|------|-----|
| Cloudflare zone | Terraform | Infrastructure — created once, long-lived |
| www → apex redirect rule | Terraform | Infrastructure — lives at the zone level |
| Worker deployment | Wrangler (`cfops -wr deploy`) | Application — changes with code |
| Custom domain attachment | Wrangler (`wrangler.toml`) | Application — Wrangler creates the DNS record and provisions SSL |
| Terraform state | R2 bucket `<RESOURCE_LABEL>-terraform-state` | Account-scoped; one bucket, one key per workspace — `<RESOURCE_LABEL>-terraform-state/dns/terraform.tfstate` etc |
| Application R2 | Per-worker bucket, e.g. `<RESOURCE_LABEL>-static-<purpose>` | Worker-scoped; each worker owns its data in isolation |
