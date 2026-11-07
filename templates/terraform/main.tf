terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4"
    }
  }

  backend "s3" {}
}

provider "cloudflare" {}

resource "cloudflare_zone" "<RESOURCE_LABEL>" {
  account_id = var.account_id
  zone       = "<DOMAIN>"
}

resource "cloudflare_record" "www" {
  zone_id = cloudflare_zone.<RESOURCE_LABEL>.id
  name    = "www"
  type    = "CNAME"
  content = "<DOMAIN>"
  proxied = true
}

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
