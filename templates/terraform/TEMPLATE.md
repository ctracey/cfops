# Terraform DNS Template

This template provisions a Cloudflare zone, www CNAME, and www→apex redirect rule. Before use, replace all placeholders with values for your project.

## Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `<DOMAIN>` | The full domain name being configured | `acme.com` |
| `<RESOURCE_LABEL>` | Terraform resource label — an internal identifier used to cross-reference resources within the config. Typically the domain shortname (no TLD). | `acme` |
| `<CLOUDFLARE_ACCOUNT_ID>` | Your Cloudflare account ID — found in the Cloudflare dashboard URL or under Account Home → Overview. Used in the R2 backend endpoint URL. | `cbf9ff5906a02000fc05409d3aee2e51` |

## Files

| File | Placeholders used |
|------|-------------------|
| `main.tf` | `<DOMAIN>`, `<RESOURCE_LABEL>` |
| `outputs.tf` | `<RESOURCE_LABEL>` |
| `backend.hcl` | `<RESOURCE_LABEL>`, `<CLOUDFLARE_ACCOUNT_ID>` |

## Example — acme.com

| Placeholder | Value |
|-------------|-------|
| `<DOMAIN>` | `acme.com` |
| `<RESOURCE_LABEL>` | `acme` |
| `<CLOUDFLARE_ACCOUNT_ID>` | *(your account ID from the Cloudflare dashboard)* |

`backend.hcl` would become:
```hcl
bucket = "acme-terraform-state"
endpoints = {
  s3 = "https://<your-account-id>.r2.cloudflarestorage.com"
}
```

`main.tf` resource block would become:
```hcl
resource "cloudflare_zone" "acme" {
  zone = "acme.com"
}
```
