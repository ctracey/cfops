# Worker DNS Config

- [1. Attach Domain to Worker](#1-attach-domain-to-worker)
  - [1.1 Worker Domain Routing Config](#11-worker-domain-routing-config)
  - [1.2 Deploy Updated Worker Config](#12-deploy-worker-config-update)
  - [1.3 Verify Worker Domain Routing](#13-verify)
- [2. Adding Further Workers](#2-adding-further-workers)

## 1. Attach Domain to Worker

This section applies when attaching a new domain route — an apex domain, a subdomain, or any route being attached for the first time.

- **New domain route:** requires DNS propagation and SSL certificate provisioning — allow up to 30 minutes
- **Redeploy on existing route** (e.g. updating worker content): instant — no propagation delay

### 1.1 Worker Domain Routing Config

Add the domain route to `wrangler.toml`:

```bash
cfops worker --attach <DOMAIN>
```

This adds the route to `wrangler.toml` automatically. Run `cfops worker --deploy` to apply it.

### 1.2 Deploy Updated Worker Config

```bash
cfops worker --deploy
```

Wrangler creates the DNS record for the apex and provisions the SSL certificate automatically. No separate DNS record management required.

### 1.3 Verify Worker Domain Routing

Allow up to 30 minutes after deploy for the DNS record to propagate and the SSL certificate to provision before running these checks. Since both the nameservers and DNS records are Cloudflare-managed, propagation is typically much faster than the industry-standard 48-hour window — but 30 minutes is a reasonable upper bound.

```bash
# Apex DNS resolves
dig <DOMAIN> @1.1.1.1

# www DNS resolves
dig www.<DOMAIN> @1.1.1.1
```

Look for A records in the ANSWER SECTION with IPs — an empty ANSWER SECTION means DNS has not propagated yet.

```bash
# Apex serves the Worker
curl -I https://<DOMAIN>

# www redirects to apex
curl -I https://www.<DOMAIN>

# Certificate is valid
curl -v https://<DOMAIN> 2>&1 | grep -A2 "SSL certificate"
```

Expected: apex returns 200, www returns 301 to `https://<DOMAIN>`, certificate issued by Cloudflare.

> **Note:** The browser may work before curl — if curl still fails with "Could not resolve host" after the dig checks pass, flush your local DNS cache (`sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder` on macOS) and try again.

## 2. Adding Further Workers

Terraform is a one-time zone setup — it does not need to change for additional workers. Each new worker just needs its domain attached and deployed:

> **Note:** Each worker is a separate deployable unit with its own repo and `wrangler.toml` — Wrangler requires a `wrangler.toml` per worker to define its name, routes, and assets. The Cloudflare account is the shared boundary — multiple workers on the same account each follow this same attach and deploy pattern from their own project directory.

```bash
cfops worker --attach sub.<DOMAIN>
cfops worker --deploy
```

Deploying that worker creates the DNS record for the subdomain in the same `<DOMAIN>` zone. Each worker manages its own DNS record independently.
