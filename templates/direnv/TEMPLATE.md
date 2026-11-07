# direnv Template

This template generates `.envrc` for a project, wiring up the Bitwarden machine account so `bws` can fetch Cloudflare credentials at runtime.

## Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `<ACCOUNT_NAME>` | Bitwarden machine account name for this project. Becomes the Keychain service name and the `BWS_PROFILE_NAME` prefix. Created in Bitwarden Secrets Manager under Machine Accounts. | `myproject-prod` |

## Example — myproject-prod

| Placeholder | Value |
|-------------|-------|
| `<ACCOUNT_NAME>` | `myproject-prod` |

`.envrc` would become:

```bash
export BWS_PROFILE_NAME=bws_myproject-prod
export BWS_ACCESS_TOKEN=$(security find-generic-password -s "$BWS_PROFILE_NAME" -w)
```
