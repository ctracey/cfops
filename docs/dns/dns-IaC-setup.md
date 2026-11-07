# DNS IaC Setup

A guide to setting up Terraform as the Infrastructure as Code tool for managing DNS config — covering credentials, state persistence, and backend access.

- [1. Tooling](#1-tooling)
- [2. Setup Cloudflare Token for DNS Config](#2-setup-cloudflare-token-for-dns-config)
- [3. Enable Terraform state persistence](#3-enable-terraform-state-persistence)
  - [3.1 Pre-requisite: enable Cloudflare object storage](#31-pre-requisite-enable-cloudflare-object-storage)
  - [3.2 Create Terraform state bucket](#32-create-terraform-state-bucket)
  - [3.3 Setup Cloudflare R2 access keys](#33-setup-cloudflare-r2-access-keys)

## 1. Tooling

> **Prerequisite:** cfops must be installed and configured before running any commands here. See [ops.md § Tooling](../ops.md#1-tooling).

> **Note:** Wrangler and Terraform use separate API tokens — intentionally scoped independently. Terraform has no worker permissions; Wrangler has no zone permissions. cfops dynamically maps the appropriate token to `CLOUDFLARE_API_TOKEN` based on the operation — `CLOUDFLARE_WORKER_PROVISION_TOKEN` for Wrangler commands, `CLOUDFLARE_DNS_CONFIG_TOKEN` for Terraform commands.

## 2. Setup Cloudflare Token for DNS Config

Create a token from the **Edit zone DNS** template, then add Transform Rules.

[Dashboard → My Profile → API Tokens → Create Token → Edit zone DNS](https://dash.cloudflare.com/profile/api-tokens)

1. Select the **Edit zone DNS** template — this pre-populates zone creation and DNS permissions
2. Add **Zone → Zone → Edit** — required to create new zones
3. Add **Zone → Single Redirect → Edit** — required for the www redirect ruleset
4. Set scope to **All zones**
5. Name the token `DNS Config (<DOMAIN>)`
6. Create the token and copy the value — shown once

Store in Bitwarden under the prod machine account as `CLOUDFLARE_DNS_CONFIG_TOKEN`.

## 3. Enable Terraform state persistence

### 3.1 Pre-requisite: enable Cloudflare object storage

R2 is Cloudflare's object storage — used here for Terraform state and future application storage. Must be activated once per account before any R2 operations are available.

(clickops — one-time, dashboard only)

**Cloudflare dashboard → left sidebar → Storage & Databases → R2 Object Storage → Add R2 subscription to my account**

Cloudflare requires manual account-level activation before any R2 operations will work. May prompt for a payment method even on the free tier.

### 3.2 Create Terraform state bucket

> **Prerequisite:** The `Worker Provision (all)` token must have R2 bucket admin permissions — verify this in the Cloudflare dashboard under **My Profile → API Tokens** before running the command. See cfops setup docs for how this token should be configured.

```bash
cfops -wr r2 bucket create <RESOURCE_LABEL>-terraform-state
```

Must exist before `terraform init` can run.

### 3.3 Setup Cloudflare R2 access keys

Terraform needs access to the R2 bucket to maintain its inventory of managed Cloudflare infrastructure — authenticated via access keys.

R2 access keys are a dedicated S3-compatible keypair — a different credential type from Cloudflare API tokens such as `Worker Provision (all)` or `DNS Config (<DOMAIN>)`. Cloudflare calls this an **R2 User Token**. It must be created through the R2 token flow.

1. Navigate to **Cloudflare dashboard → left sidebar → Storage & Databases → R2 Object Storage → Manage API Tokens**
2. Click **Create API Token**
3. Set Permissions to **Object Read & Write**, Bucket to **`<RESOURCE_LABEL>-terraform-state`**
4. Copy the Token, Access Key ID, and Secret Access Key — all shown once
5. Store all three in Bitwarden under the prod machine account project:

| Secret name | Value |
|-------------|-------|
| `CLOUDFLARE_R2_TERRAFORM_STATE_TOKEN` | Token value |
| `CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID` | Access Key ID |
| `CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY` | Secret Access Key |
