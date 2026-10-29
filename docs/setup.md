# Setup

## 1. Environment Setup (Mise)

[mise](https://mise.jdx.dev/) installs and activates `cfops` per-project without adding it to the project repo or installing it globally on the machine.

### 1.1 Dependencies

| Tool | Purpose | Install |
|------|---------|---------|
| [mise](https://mise.jdx.dev/) | Per-project tool management | `brew install mise` |

### 1.2 Setup Shell

Add to `~/.zshrc` (once per machine):

```zsh
eval "$(mise activate zsh)"
```

This hooks `cd` so mise can activate per-project tools automatically when you enter a directory.

## 2. Project Setup (Dependency)

Declare `cfops` as a project dependency so mise makes it available automatically when working in the project.

### 2.1 Config

Add the following to your project's `.mise.toml`:

```toml
[tools]
direnv = "latest"
terraform = "latest"
"npm:wrangler" = "latest"
bitwarden-secrets-manager = "latest"
"github:ctracey/cfops" = "latest"
```

If you don't have a `.mise.toml` yet, download the template from this repo:

```bash
curl -o .mise.toml https://raw.githubusercontent.com/ctracey/cfops/main/templates/.mise.toml
```

### 2.2 Install cfops CLI

```bash
mise install
```

`cfops` will be available on `$PATH` whenever you're inside that project directory.

> **NOTE:** By design, `cfops` only works in directories with a `.mise.toml` that declares it as a dependency. Running it outside that context will produce a mise error — this is expected behaviour.

### 2.3 Keep cfops Updated

To upgrade `cfops` and all other `latest` tools to their newest versions:

```bash
mise upgrade
```
