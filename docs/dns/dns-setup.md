# DNS Setup Guide

A guide to setting up Terraform as the Infrastructure as Code tool for managing DNS config — covering credentials, state persistence, and backend access.

- [1. Tooling](#1-tooling)
- [2. Setup Cloudflare Token for DNS Config](#2-setup-cloudflare-token-for-dns-config)
- [3. Enable Terraform state persistence](#3-enable-terraform-state-persistence)
  - [3.1 Pre-requisite: enable Cloudflare object storage](#31-pre-requisite-enable-cloudflare-object-storage)
  - [3.2 Create Terraform state bucket](#32-create-terraform-state-bucket)
  - [3.3 Setup Cloudflare R2 access keys](#33-setup-cloudflare-r2-access-keys)
- [4. DNS setup complete](#4-dns-setup-complete)

## 1. Tooling

> **Prerequisite:** cfops must be installed and configured before running any commands here. See [Installation Guide](../installation.md).

> **Note:** Wrangler and Terraform use separate API tokens — intentionally scoped independently. Terraform has no worker permissions; Wrangler has no zone permissions. cfops dynamically maps the appropriate token to `CLOUDFLARE_API_TOKEN` based on the operation — `CLOUDFLARE_WORKER_PROVISION_TOKEN` for Wrangler commands, `CLOUDFLARE_DNS_CONFIG_TOKEN` for Terraform commands.

## 2. Setup Cloudflare Token for DNS Config

Here we configure the DNS token for your domain — scoping it to the correct zone and adding the permissions Terraform needs.

1. Find the `DNS config token (<DOMAIN>)` token created in [setup 1.2](../setup.md#12-cloudflare-access) ([Dashboard → My Profile → API Tokens](https://dash.cloudflare.com/profile/api-tokens))
2. Edit the token — within the zone policy section, expand each group and enable the following permissions:

   | Group | Permission | Access |
   |---|---|---|
   | DNS & Zones | DNS | Edit (pre-set by template) |
   | DNS & Zones | Zone | Edit |
   | Rules & Configuration | Single Redirect | Edit |

3. Click **Continue to summary** then **Update token**

> **Note:** The token value does not change when editing permissions — no need to update Bitwarden. `CLOUDFLARE_DNS_CONFIG_TOKEN` was already stored in setup [1.3](../setup.md#13-store-credentials-bitwarden).

## 3. Enable Terraform state persistence

### 3.1 Pre-requisite: enable Cloudflare object storage

R2 is Cloudflare's object storage — used here for Terraform state and future application storage. Must be activated once per account before any R2 operations are available.

(clickops — one-time, dashboard only)

**Cloudflare dashboard → left sidebar → Storage & Databases → R2 Object Storage → Add R2 subscription to my account**

Cloudflare requires manual account-level activation before any R2 operations will work. May prompt for a payment method even on the free tier.

> **Note:** Once R2 is activated, the **Create bucket** option will appear in the R2 Object Storage dashboard.

### 3.2 Create Terraform state bucket

> **Prerequisite:** The `Worker provisioning token` created in [setup 1.2](../setup.md#12-cloudflare-access) must have R2 bucket admin permissions.
>
> The token should be correctly configured from the template — optionally verify by checking **Account policy → Workers R2 Storage → Edit** is enabled ([Dashboard → My Profile → API Tokens](https://dash.cloudflare.com/profile/api-tokens)).

Create the R2 bucket that Terraform will use to store state:

```bash
cfops -wr r2 bucket create <DOMAIN>-terraform-state
```

Must exist before `terraform init` can run.

### 3.3 Setup Cloudflare R2 access keys

Terraform needs access to the R2 bucket to maintain its inventory of managed Cloudflare infrastructure — authenticated via access keys.

R2 access keys are a dedicated S3-compatible keypair — a different credential type from Cloudflare API tokens such as `Worker Provision (all)` or `DNS Config (<DOMAIN>)`. Cloudflare calls this an **R2 User Token**. It must be created through the R2 token flow.

1. Navigate to **Cloudflare dashboard → left sidebar → Storage & Databases → R2 Object Storage → Manage API Tokens**
2. Click **Create API Token**
3. Set **Permissions** to **Object Read & Write**
4. For **Specify bucket(s)**, select **Apply to specific buckets only** and specify **`<DOMAIN>-terraform-state`**
5. Click **Create User API Token**
6. Copy the Token, Access Key ID, and Secret Access Key — all shown once
7. Store all three in Bitwarden as secrets for the prod machine account project:

| Secret name | Value |
|-------------|-------|
| `CLOUDFLARE_R2_TERRAFORM_STATE_TOKEN` | Token value |
| `CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID` | Access Key ID |
| `CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY` | Secret Access Key |

## 4. DNS setup complete

Return to and complete the [Setup Guide](../setup.md).
