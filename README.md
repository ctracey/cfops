# cfops

CLI tooling for Cloudflare operations, distributed via [mise](https://mise.jdx.dev/).

## Overview

`cfops` is a credential-injecting wrapper around Wrangler and Terraform for managing Cloudflare Workers infrastructure.

- Distributed as a [mise](https://mise.jdx.dev/) tool: projects declare it as a dependency, and mise activates it on `$PATH` only when inside that project directory.
- Project-specific Cloudflare details are maintained in the project's `.envrc`.

## Guides

- [Setup](docs/setup.md)
- [Usage](docs/usage.md)
- [Pipeline](docs/pipeline.md)
