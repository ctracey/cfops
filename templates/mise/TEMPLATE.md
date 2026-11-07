# mise Template

This template declares per-project tool dependencies for [mise](https://mise.jdx.dev/). No placeholders — copy `.mise.toml` to your project root and run `mise install`.

## Tools

| Tool | Purpose |
|------|---------|
| `direnv` | Loads `.envrc` automatically on `cd` — activates Bitwarden credentials per project |
| `terraform` | Infrastructure as Code — manages Cloudflare zone and DNS config |
| `npm:wrangler` | Cloudflare Workers CLI — deploy, delete, manage workers |
| `bitwarden-secrets-manager` | `bws` CLI — fetches secrets from Bitwarden Secrets Manager at runtime |
| `github:ctracey/cfops` | cfops CLI — wraps Wrangler and Terraform with credential injection |

## Usage

```bash
# Download template (skips if .mise.toml already exists)
[ -f .mise.toml ] && echo "ACTION: .mise.toml already exists, manually update .mise config" || curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/mise/.mise.toml

mise install
```

To pin `cfops` to a specific version rather than tracking `latest`:

```toml
"github:ctracey/cfops" = "0.7.0"
```
