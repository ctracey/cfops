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
"ubi:ctracey/cfops" = "latest"
```

### 2.2 Use

```bash
mise install
```

`cfops` will be available on `$PATH` whenever you're inside that project directory.
