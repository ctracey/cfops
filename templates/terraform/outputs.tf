output "nameservers" {
  description = "Cloudflare nameservers — set these at the domain registrar"
  value       = cloudflare_zone.<RESOURCE_LABEL>.name_servers
}
