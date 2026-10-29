# SecOps

Covers credentials, access setup, and secret management for this project. For tooling and deployment see [ops.md](ops.md). For platform and application structure see [architecture.md](architecture.md).

- [1. Cloudflare Access](#1-cloudflare-access)
- [2. Switching Environments](#2-switching-environments)
  - [1.1 Setup Access Credentials (Cloudflare)](#11-setup-access-credentials-cloudflare)
    - [1.1.1 Account ID](#111-account-id)
    - [1.1.2 API Token](#112-api-token)
  - [1.2 Store Credentials Securely (Bitwarden Secret Manager)](#12-store-credentials-securely-bitwarden-secret-manager)
  - [1.3 Environment Var Config](#13-environment-var-config)
  - [1.4 Secret Manager Token - Local Storage (MacOS Keychain Access)](#14-secret-manager-token---local-storage-macos-keychain-access)
  - [1.5 Authorised Cloudflare Access (bws cli tool)](#15-authorised-cloudflare-access-bws-cli-tool)

## 1. Cloudflare Access

Secrets are stored in Secrets Manager (Bitwarden) and fetched at runtime via `bws`. Access flows: Keychain → bws → Cloudflare credentials.

[dash.cloudflare.com](https://dash.cloudflare.com)

### 1.1 Setup Access Credentials (Cloudflare)

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

1. Create project `<BITWARDEN_PROJECT_NAME>`
2. Add secret `CLOUDFLARE_API_TOKEN` with the token from 1.1.2 as its value
3. Add secret `CLOUDFLARE_ACCOUNT_ID` with the account ID from 1.1.1 as its value
4. Create machine account `<BWS_MACHINE_ACCOUNT_NAME>`, grant it read access to the project
5. Generate its access token — copy it now, it is only shown once. Store it temporarily (e.g. a notes app) — you will add it to Keychain in 1.4

### 1.3 Environment Var Config

Create `.envrc` in the project root:

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

### 1.4 Secret Manager Token - Local Storage (MacOS Keychain Access)

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

### 1.5 Authorised Cloudflare Access (bws cli tool)

Credentials are fetched from Bitwarden and injected as env vars for a single command, with nothing persisted to the shell.

Secret names in Bitwarden become env var names — `CLOUDFLARE_API_TOKEN` in Bitwarden is injected as `$CLOUDFLARE_API_TOKEN` for the duration of the command. Without `bws run`, those vars don't exist in the shell.

Verify credentials are injected correctly — `wrangler whoami` returns an auth error without `bws run`, and the authenticated account with it:

```bash
wrangler whoami              # error: missing or invalid API token
bws run -- wrangler whoami   # shows authenticated Cloudflare account
wrangler whoami              # error: missing or invalid API token
```

Add a `cf` function to `~/.zshrc`. It triggers direnv to load `BWS_ACCESS_TOKEN` from Keychain if it isn't already set, then injects it for the duration of the command only — nothing persists to the shell session:

```bash
cf() {
  [ -z "$BWS_ACCESS_TOKEN" ] && eval "$(direnv export zsh)"
  case $1 in
    -tf) shift; bws run -- terraform "$@" ;;
    -wr) shift; bws run -- wrangler "$@" ;;
      *) echo "Usage: cf -tf|-wr [args]" ;;
  esac
}
```

Usage: `cf -tf apply`, `cf -wr deploy`.

## 2. Switching Environments

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
