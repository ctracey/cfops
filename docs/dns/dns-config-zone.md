# Config Cloudflare Zone DNS

Terraform manages all DNS configuration for `<DOMAIN>`, eliminating clickops wherever possible.

- [1. Prerequisite: DNS IaC Setup](#1-prerequisite-dns-iac-setup)
- [2. Cloudflare Zone & DNS Config](#2-cloudflare-zone--dns-config)
  - [2.1 Config definition](#21-config-definition)
  - [2.2 www redirect config](#22-www-redirect-config)
  - [2.3 Apply config](#23-apply-config)
- [3. Delegate Nameservers at Domain Registrar](#3-delegate-nameservers-at-domain-registrar)
  - [3.1 Retrieve Cloudflare nameservers](#31-retrieve-cloudflare-nameservers)
  - [3.2 Update nameservers at registrar](#32-update-nameservers-at-registrar)
  - [3.3 Verify](#33-verify)
- [4. Narrow DNS Token Scope](#4-narrow-dns-token-scope)
- [5. Zone setup complete](#5-zone-setup-complete)

## 1. Prerequisite: DNS IaC Setup

Before running any Terraform commands, complete the [DNS IaC Setup](dns-setup.md) — this provisions the Cloudflare token and R2 state backend that Terraform depends on.

## 2. Cloudflare Zone & DNS Config

Apply Terraform config to provision the Cloudflare zone and DNS records for `<DOMAIN>`.

A Cloudflare zone is the container for a domain's DNS settings — it controls how traffic for `<DOMAIN>` is routed. Configuring DNS maps the domain to the right services (workers, redirects) so requests reach their intended destination.

### 2.1 Config definition

The Terraform config is scaffolded by `cfops zone --init` into `terraform/`. It provisions the `<DOMAIN>` zone, a www CNAME record, and a www→apex 301 redirect ruleset.

### 2.2 www redirect config

Managed by Terraform via a `cloudflare_ruleset` resource. The redirect rule lives at the zone level and requires no Worker logic.

The Terraform resource redirects `www.<DOMAIN>/*` → `https://<DOMAIN>/$1` with a 301. This is applied as part of the same `terraform apply` run as the zone.

```hcl
resource "cloudflare_ruleset" "www_redirect" {
  zone_id     = cloudflare_zone.<RESOURCE_LABEL>.id
  name        = "www to apex redirect"
  kind        = "zone"
  phase       = "http_request_dynamic_redirect"

  rules {
    action      = "redirect"
    description = "Redirect www.<DOMAIN> to <DOMAIN>"
    enabled     = true
    expression  = "(http.host eq \"www.<DOMAIN>\")"

    action_parameters {
      from_value {
        status_code           = 301
        preserve_query_string = true
        target_url {
          expression = "concat(\"https://<DOMAIN>\", http.request.uri.path)"
        }
      }
    }
  }
}
```

### 2.3 Apply config

```bash
cd terraform
cfops -tf init -backend-config=./backend.hcl
cfops -tf plan
cfops -tf apply
```

Terraform creates the `<DOMAIN>` zone and outputs the two assigned Cloudflare nameservers.

## 3. Delegate Nameservers at Domain Registrar

### 3.1 Retrieve Cloudflare nameservers

To retrieve the assigned nameservers at any time:

```bash
cd terraform
cfops -tf output
```

### 3.2 Update nameservers at registrar

**Manual step** — this cannot be automated.

In your domain registrar's control panel, navigate to the nameserver settings for `<DOMAIN>` and replace the existing nameservers with the two Cloudflare nameservers from the Terraform output.

Ensure DNSSEC is off at the registrar (Cloudflare manages DNSSEC from their side if enabled later).

Propagation typically takes minutes to a few hours.

### 3.3 Verify

Confirm nameserver delegation is in place:

```bash
dig NS <DOMAIN>
```

The zone becomes active in Cloudflare once it detects the delegation.

## 4. Narrow DNS Token Scope

Now that `<DOMAIN>` exists as a Cloudflare zone, narrow the DNS token from "All zones from account" to this specific zone.

1. Find the `DNS config token (<DOMAIN>)` token (**Dashboard → Manage Account → Account API Tokens**)
2. Edit the token
3. Under **Zone policy**, deselect the account — **Specific zone** is not selectable until the account is removed
4. Change from **All zones from account** to **Specific zone** and select `<DOMAIN>`
5. Click **Continue to summary** then **Update token**

## 5. Zone setup complete

The `<DOMAIN>` zone is now provisioned in Cloudflare with DNS records and nameserver delegation in place.

> **Note:** Nothing will be rendered until a worker is attached to the domain. If you have existing workers, they will not serve traffic on `<DOMAIN>` until they are explicitly attached.

**Next: [Attach a worker to the domain →](./dns-config-worker.md)**
