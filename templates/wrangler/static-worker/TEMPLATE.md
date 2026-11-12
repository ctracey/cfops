# Wrangler Static Worker Template

Configures a Cloudflare Worker that serves static assets from `dist/`. Deploys to `*.workers.dev` — for custom domain routing see [dns-config-worker.md](../../../docs/dns/dns-config-worker.md). Before use, replace all placeholders with values for your project.

## Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `<WORKER_NAME>` | Worker name — follow the [naming convention](../../../docs/usage-scenarios.md#21-worker-naming-convention) | `mysite-static` |
| `<COMPATIBILITY_DATE>` | Wrangler compatibility date — use today's date | `2026-11-09` |
| `<ASSETS_DIRECTORY>` | Path to the built static assets directory | `./dist` |
