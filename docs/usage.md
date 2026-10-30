# Usage

Covers credentials, access setup, and secret management for this project. For tooling and deployment see [ops.md](ops.md). For platform and application structure see [architecture.md](architecture.md).

- [1. Config Cloudflare Credentials](#1-config-cloudflare-credentials)
  - [1.1 Setup Access (Cloudflare)](#11-setup-access-cloudflare)
  - [1.2 Store Credentials Securely (Bitwarden Secret Manager)](#12-store-credentials-securely-bitwarden-secret-manager)
  - [1.3 Configure Cloudflare Profile (direnv)](#13-configure-cloudflare-profile-direnv)
  - [1.4 Store Secret Manager Token (MacOS Keychain)](#14-store-secret-manager-token-macos-keychain)
- [2. Enable Cloudflare Authorisation](#2-enable-cloudflare-authorisation)
  - [2.1 Secure Secret Usage (bws)](#21-secure-secret-usage-bws)
  - [2.2 Authorised Cloudflare Operations (mise: cfops)](#22-authorised-cloudflare-operations-mise-cfops)
- [3. Switching Environments](#3-switching-environments)
- [4. Setting Up Additional Projects](#4-setting-up-additional-projects)

## 1. Config Cloudflare Credentials

Secrets are stored in Secrets Manager (Bitwarden) and fetched at runtime via `bws`. Access flows: Keychain → bws → Cloudflare credentials.

### 1.1 Setup Access (Cloudflare)

Two pieces of information needed from the Cloudflare dashboard.

#### 1.1.1 Account ID

Your Account ID is in the URL of any dashboard page: `dash.cloudflare.com/<account-id>/...`

#### 1.1.2 API Token

Create an API token scoped to Worker script edits.

- [Dashboard → My Profile → API Tokens → Create Token](https://dash.cloudflare.com/profile/api-tokens)
- Use the **Edit Cloudflare Workers** template (Account → Workers Scripts → Edit)

### 1.2 Store Credentials Securely (Bitwarden Secret Manager)

Store the Cloudflare credentials as secrets and create a machine account per device to read them.

In the [Bitwarden web app](https://vault.bitwarden.com), switch to Secrets Manager via the product switcher (set up the free organisation if prompted):

1. Create project `<BITWARDEN_PROJECT_NAME>` (e.g. `mydomain-cloudflare-staging` or `mydomain-cloudflare-prod`)
2. Add secret `CLOUDFLARE_API_TOKEN` with the token from 1.1.2 as its value
3. Add secret `CLOUDFLARE_ACCOUNT_ID` with the account ID from 1.1.1 as its value
4. Create machine account `<BWS_MACHINE_ACCOUNT_NAME>`, grant it read access to the project
5. Generate its access token — copy it now, it is only shown once. Store it temporarily (e.g. a notes app) — you will add it to Keychain in 1.4

### 1.3 Configure Cloudflare Profile (direnv)

Configure the Bitwarden profile name for this project so `bws` can retrieve the intended Cloudflare account secrets at runtime.

Using [direnv](https://direnv.net/) we can create env vars based on a config file.

First, exclude `.envrc` from version control:

```bash
echo ".envrc" >> .gitignore
```

Then create `.envrc` in the project root with the Bitwarden profile name for this project:

```bash
export BWS_PROFILE_NAME=bws_<your-BWS_MACHINE_ACCOUNT_NAME>
```

Allow direnv to run for this project (required each time `.envrc` changes):

```bash
direnv allow
```

Verify `BWS_PROFILE_NAME` is exported:

```bash
eval "$(direnv export zsh)"
echo BWS_PROFILE_NAME: $BWS_PROFILE_NAME
```

Expected output: `bws_` followed by the value you set for `<BWS_MACHINE_ACCOUNT_NAME>`. If you see that, direnv is working correctly.

### 1.4 Store Secret Manager Token (MacOS Keychain)

Store the Secrets Manager (Bitwarden) machine account token in Keychain, requiring your macOS password each time the `bws` CLI reads it.

The token is the machine account access token generated in 1.2. To retrieve or regenerate it:

- [Bitwarden Secrets Manager](https://vault.bitwarden.com) → Machine Accounts → `<BWS_PROFILE_NAME>` → Access Tokens → Add Access Token
- Copy it immediately — it is only shown once

Store the machine account access token in the Keychain using `$BWS_PROFILE_NAME` set in 1.3. Ensure it is loaded first, then add to Keychain — the command will prompt for the token, paste the one temporarily stored in 1.2:

```bash
eval "$(direnv export zsh)"
security add-generic-password -a "$USER" -s "$BWS_PROFILE_NAME" -w
```

Then configure access control:

1. Keychain Access → right-click `$BWS_PROFILE_NAME` → Get Info → Access Control
2. Check **"Confirm before allowing access"**
3. In the **"Always allow access by these applications"** list, select `security` and remove it (click the `−` button)
4. Save Changes

When you added the item via the `security` CLI, macOS automatically trusted that binary — removing it forces every read, including from the terminal, to prompt for your macOS password. Nothing reads it silently.

Add this line to `.envrc`:

```bash
export BWS_ACCESS_TOKEN=$(security find-generic-password -s "$BWS_PROFILE_NAME" -w)
```

Allow and reload direnv:

```bash
direnv allow
eval "$(direnv export zsh)"
```

Verify the token was loaded:

```bash
echo BWS_ACCESS_TOKEN: $BWS_ACCESS_TOKEN
```

## 2. Enable Cloudflare Authorisation

To access [Cloudflare](https://dash.cloudflare.com), both Wrangler and Terraform expect `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` to be set — these must match the secret names configured in 1.2.

### 2.1 Secure Secret Usage (bws)

Credentials are fetched from Bitwarden and injected as env vars for a single command, with nothing persisted to the shell.

Secret names in Bitwarden become env var names — `CLOUDFLARE_API_TOKEN` in Bitwarden is injected as `$CLOUDFLARE_API_TOKEN` for the duration of the command. Without `bws run`, those vars don't exist in the shell.

Verify credentials are injected correctly — `wrangler whoami` returns an auth error without `bws run`, and the authenticated account with it:

```bash
wrangler whoami              # error: missing or invalid API token
bws run -- wrangler whoami   # shows authenticated Cloudflare account
wrangler whoami              # error: missing or invalid API token
```

### 2.2 Authorised Cloudflare Operations (mise: cfops)

`cfops` injects Cloudflare credentials from Bitwarden Secrets Manager before delegating to Wrangler or Terraform. Credentials are never stored on disk or exported to the global environment.

**Dashboard**

`-ls` — opens the Workers & Pages dashboard for the active account in the browser

```bash
cfops -ls
```

**Wrangler**

`-wr` — delegates to `wrangler` with credentials injected

```bash
cfops -wr deploy                  # Deploy the worker
cfops -wr deployments status      # Check deployment status
cfops -wr delete                  # Delete the worker (prompts for confirmation)
```

**Terraform**

`-tf` — delegates to `terraform` with credentials injected

```bash
cfops -tf init
cfops -tf plan
cfops -tf apply
```

## 3. Switching Environments

Each environment (staging, prod) is a separate Cloudflare account with its own Bitwarden machine account and API token. The active environment is determined by `BWS_PROFILE_NAME` in `.envrc`.

To switch, update `BWS_PROFILE_NAME` to the machine account for the target environment and reload:

```bash
# in .envrc
export BWS_PROFILE_NAME=bws_<target-BWS_MACHINE_ACCOUNT_NAME>
```

```bash
direnv allow
eval "$(direnv export zsh)"
```

`wrangler.toml` does not change — the same config deploys to whichever account the credentials belong to.

## 4. Setting Up Additional Projects

Once the initial setup in sections 1 and 2 is complete, setting up cfops for additional projects is straightforward.

```bash
[ -f .mise.toml ] && echo -e "\033[34m→\033[0m ACTION: .mise.toml already exists, manually update .mise config" || curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/.mise.toml
mise install
cfops init
direnv allow
cfops -wr whoami
```

