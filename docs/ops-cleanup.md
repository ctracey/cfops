# Ops Cleanup

Steps to fully tear down a cfops-managed Cloudflare deployment.

## 1. Permanent / cannot be undone

- **R2 subscription** — once enabled on a Cloudflare account it cannot be removed
- **workers.dev subdomain** — account-level, cannot be unset, only changed

## 2. Worker and infrastructure

```bash
cfops -wr delete
```

Deletes the Cloudflare Worker. Prompts for confirmation.

```bash
cfops -tf init -backend-config=backend.hcl
cfops -tf destroy
```

Destroys Terraform-managed resources (zone, DNS records, redirect rules).

## 3. R2 Buckets

Delete all R2 buckets created for this project from **Cloudflare dashboard → Storage & Databases → R2 Object Storage**:

- `<RESOURCE_LABEL>-terraform-state`

> **Note:** The R2 subscription itself cannot be removed once enabled — see [section 1](#1-permanent--cannot-be-undone).

## 4. Cloudflare tokens

Delete all three from **Cloudflare dashboard → My Profile → API Tokens**:

- `Worker Provision (all)` — Wrangler token
- `DNS Config (<domain>)` — Terraform token
- R2 User Token for terraform state bucket (also accessible via R2 → Manage API Tokens)

## 5. Bitwarden

In Bitwarden Secrets Manager, remove:

- Machine account
- All secrets (`CLOUDFLARE_WORKER_PROVISION_TOKEN`, `CLOUDFLARE_DNS_CONFIG_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`, `CLOUDFLARE_R2_TERRAFORM_STATE_TOKEN`, `CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID`, `CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY`)
- Project

## 6. Keychain

Delete the machine account token (item name has `bws_` prefix):

```bash
security delete-generic-password -s "bws_<machine-account-name>"
```
