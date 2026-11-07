# Setup Guide

Per-project configuration — credentials, project init, and IaC setup. Gets the project to a ready state for [usage](usage.md).

- [1. Credentials](#1-credentials)
- [2. Project Init](#2-project-init)
- [3. DNS IaC Setup](#3-dns-iac-setup)

## 1. Credentials

Credentials are stored in Bitwarden Secrets Manager and fetched at runtime via `bws`. Access flows: Keychain → bws → Cloudflare credentials.

### 1.1 Cloudflare Access

**Account ID** — in the URL of any dashboard page: `dash.cloudflare.com/<account-id>/...`

**API Tokens** — two tokens, each scoped to the minimum permissions for its purpose:

- **Worker provisioning token** (Wrangler): [Dashboard → My Profile → API Tokens → Create Token](https://dash.cloudflare.com/profile/api-tokens) → **Edit Cloudflare Workers** template
- **DNS config token** (Terraform): [Dashboard → My Profile → API Tokens → Create Token](https://dash.cloudflare.com/profile/api-tokens) → **Edit zone DNS** template (see [DNS IaC Setup](dns/dns-IaC-setup.md) for required permissions)

### 1.2 Store Credentials (Bitwarden)

In the [Bitwarden web app](https://vault.bitwarden.com), switch to Secrets Manager:

1. Create project `<BITWARDEN_PROJECT_NAME>` (e.g. `mydomain-cloudflare-prod`)
2. Add secret `CLOUDFLARE_WORKER_PROVISION_TOKEN`
3. Add secret `CLOUDFLARE_DNS_CONFIG_TOKEN`
4. Add secret `CLOUDFLARE_ACCOUNT_ID`
5. Create machine account `<BWS_MACHINE_ACCOUNT_NAME>`, grant it read access to the project
6. Generate its access token — shown once, store temporarily until added to Keychain in 1.3

### 1.3 Store Machine Account Token (Keychain)

Add the machine account token to Keychain (the command will prompt for the token value):

```bash
security add-generic-password -a "$USER" -s "bws_<BWS_MACHINE_ACCOUNT_NAME>" -w
```

Then configure access control so every read requires your macOS password:

1. Keychain Access → right-click `bws_<BWS_MACHINE_ACCOUNT_NAME>` → Get Info → Access Control
2. Check **"Confirm before allowing access"**
3. In **"Always allow access by these applications"**, select `security` and remove it
4. Save Changes

## 2. Project Init

Run `cfops init` to scaffold the project:

```bash
cfops init
direnv allow
```

`cfops init` prompts for:
- Bitwarden machine account name → writes `.envrc`
- Domain → scaffolds `terraform/` with substituted config files

`.envrc` is added to `.gitignore` automatically.

Verify credentials are working:

```bash
cfops -wr whoami
```

## 3. DNS IaC Setup

Before running any Terraform commands, complete the [DNS IaC Setup](dns/dns-IaC-setup.md) — this provisions the R2 state bucket and R2 access keys that Terraform depends on.

## 4. Setup Complete

cfops is now setup and ready for use. Next step is to follow the [usage guide](usage.md).
