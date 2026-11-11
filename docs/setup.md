# Setup Guide

Per-project configuration — credentials, project init, and IaC setup. Gets the project to a ready state for [usage](usage.md).

- [1. Credentials](#1-credentials)
- [2. DNS IaC Setup](#2-dns-iac-setup)
- [3. Project Init](#3-project-init)

## 1. Credentials

Credentials are stored in Bitwarden Secrets Manager and fetched at runtime via `bws`. Access flows: Keychain → bws → Cloudflare credentials.

### 1.1 Cloudflare Account

If you don't already have one, create a [Cloudflare](https://cloudflare.com) account.

### 1.2 Cloudflare Access

**Account ID** — in the URL of any dashboard page: `dash.cloudflare.com/<account-id>/...`

**API Tokens** — create both tokens via [Dashboard → My Profile → API Tokens → Create Token](https://dash.cloudflare.com/profile/api-tokens):

| Token Name | Purpose | Template | Expiration | Account policy | Zone policy | Config |
|---|---|---|---|---|---|---|
| **Worker provisioning token** | Wrangler | Edit Cloudflare Workers | 1 year | Select account | All zones from account | template defaults are sufficient |
| **DNS config token (`<DOMAIN>`)** | Terraform | Edit zone DNS | 1 year | N/A | All zones from account | see [DNS IaC Setup](dns/dns-setup.md) for required permissions |

### 1.3 Store Credentials (Bitwarden)

In the [Bitwarden web app](https://vault.bitwarden.com), switch to Secrets Manager:

1. Create project `<BITWARDEN_PROJECT_NAME>` (e.g. `mydomain-cloudflare-prod`)
2. Add secret `CLOUDFLARE_WORKER_PROVISION_TOKEN`
3. Add secret `CLOUDFLARE_DNS_CONFIG_TOKEN`
4. Add secret `CLOUDFLARE_ACCOUNT_ID`
5. Create machine account `<BWS_MACHINE_ACCOUNT_NAME>` (e.g username-devicename-YYYY)
6. Grant the machine account read access to `<BITWARDEN_PROJECT_NAME>`
7. Generate an access token for the machine account with a 1 year expiration (shown once, store temporarily until added to Keychain in 1.4)

### 1.4 Store Machine Account Token (macOS Keychain)

Add the machine account token to macOS Keychain (the command will prompt for the token value):

```bash
security add-generic-password -a "$USER" -s "bws_<BWS_MACHINE_ACCOUNT_NAME>" -p "<TOKEN_VALUE>"
```

Then configure access control so every read requires your macOS password:

1. Open the macOS Keychain Access application
2. Find `bws_<BWS_MACHINE_ACCOUNT_NAME>`
3. Right-click → Get Info → Access Control
4. Check **"Confirm before allowing access"**
5. In **"Always allow access by these applications"**, select `security` and remove it
6. Save Changes

## 2. DNS IaC Setup

If you're going to use cfops for DNS config, set up DNS IaC by following the [DNS Setup Guide](dns/dns-setup.md).

## 3. Project Init

Run `cfops init` to scaffold the project:

```bash
cfops init
direnv allow
```

`cfops init` prompts for:
- Bitwarden machine account name → writes `.envrc`
- Whether to set up DNS for a domain — if yes, prompts for domain and scaffolds `terraform/`

`.envrc` is added to `.gitignore` automatically.

Verify credentials are working:

```bash
cfops -wr whoami
```

## 4. Setup Complete

cfops is now setup and ready for use. Next step is to follow the [usage guide](usage-orientation.md).
