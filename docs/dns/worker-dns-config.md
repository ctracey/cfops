# Worker DNS Config

- [1. Attach Domain to Worker](#1-attach-domain-to-worker)
  - [1.1 Configure](#11-configure)
  - [1.2 Deploy](#12-deploy)
  - [1.3 Verify](#13-verify)
- [2. Adding Further Workers](#2-adding-further-workers)

## 1. Attach Domain to Worker

### 1.1 Configure

Add to `wrangler.toml`:

```toml
routes = [
  { pattern = "<DOMAIN>", custom_domain = true }
]
```

### 1.2 Deploy

```bash
cfops -wr deploy
```

Wrangler creates the DNS record for the apex and provisions the SSL certificate automatically. No separate DNS record management required.

### 1.3 Verify

```bash
# Zone delegation
dig NS <DOMAIN>

# Apex serves the Worker
curl -I https://<DOMAIN>

# www redirects to apex
curl -I https://www.<DOMAIN>

# Certificate is valid
curl -v https://<DOMAIN> 2>&1 | grep -A2 "SSL certificate"
```

Expected: apex returns 200, www returns 301 to `https://<DOMAIN>`, certificate issued by Cloudflare.

## 2. Adding Further Workers

Terraform is a one-time zone setup — it does not need to change for additional workers. Each new worker just needs its own `wrangler.toml` with the appropriate route:

```toml
routes = [
  { pattern = "sub.<DOMAIN>", custom_domain = true }
]
```

Deploying that worker creates the DNS record for the subdomain in the same `<DOMAIN>` zone. Each worker manages its own DNS record independently.
