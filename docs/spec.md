# cfops Acceptance Test Spec

Concrete test cases for verifying cfops behaviour. Run against the bash implementation to confirm, then against the Rust binary to verify parity.

Each case lists: command, pre-conditions, expected stdout, expected stderr, expected exit code.

---

## `-v`

### version string

```
command:  cfops -v
stdout:   cfops 0.6.1
stderr:   (empty)
exit:     0
```

---

## `-h` / `--help`

### short flag

```
command:  cfops -h
stdout:   contains "cfops"
          contains "init"
          contains "deploy"
          contains "delete"
          contains "list"
          contains "-wr"
          contains "--wrangler"
          contains "-tf"
          contains "--terraform"
exit:     0
```

### long flag

```
command:  cfops --help
stdout:   same as -h
exit:     0
```

---

## unknown command

### unrecognised subcommand

```
command:  cfops foobar
stdout:   (empty)
stderr:   contains "Unknown command"
          contains "cfops -h"
exit:     1
```

### no arguments

```
command:  cfops
stdout:   (empty)
stderr:   contains "Unknown command"
          contains "cfops -h"
exit:     1
```

---

## `init`

### creates .envrc when none exists

```
pre:      no .envrc in cwd
          no .gitignore in cwd
input:    "myaccount" at prompt
command:  cfops init
stdout:   contains "✓ .envrc created"
          contains "✓ .envrc added to .gitignore"
          contains "direnv allow"
          contains "cfops -wr whoami"
exit:     0
post:     .envrc exists in cwd
          .envrc contains "BWS_PROFILE_NAME=bws_myaccount"
          .envrc contains "security find-generic-password"
          .gitignore contains ".envrc"
```

### skips when .envrc already exists

```
pre:      .envrc exists in cwd
command:  cfops init
stdout:   contains "already exists"
stderr:   (empty)
exit:     0
post:     .envrc unchanged
```

### appends .envrc to existing .gitignore

```
pre:      no .envrc in cwd
          .gitignore exists but does not contain ".envrc"
input:    "myaccount" at prompt
command:  cfops init
post:     .gitignore contains ".envrc"
```

### does not duplicate .envrc in .gitignore

```
pre:      no .envrc in cwd
          .gitignore exists and already contains ".envrc"
input:    "myaccount" at prompt
command:  cfops init
post:     .gitignore contains ".envrc" exactly once
```

### warns if node < 22

```
pre:      node is on PATH at version < 22
input:    "myaccount" at prompt
command:  cfops init
stdout:   contains "node"
          contains "wrangler requires node 22"
```

### no node warning if node >= 22

```
pre:      node is on PATH at version >= 22
input:    "myaccount" at prompt
command:  cfops init
stdout:   does not contain "wrangler requires node 22"
```

---

## credential loading (shared behaviour)

These apply to: `list`, `deploy`, `delete`, `-wr`, `-tf`

### exits with error if BWS_ACCESS_TOKEN unavailable

```
pre:      BWS_ACCESS_TOKEN not set
          direnv not available or .envrc not present
command:  cfops -wr whoami
stdout:   (empty)
stderr:   contains "BWS_ACCESS_TOKEN could not be loaded"
          contains "BWS_PROFILE_NAME"
          contains "docs/setup.md"
exit:     1
```

---

## `-wr` / `--wrangler`

### passes args to wrangler with CLOUDFLARE_API_TOKEN mapped

```
pre:      BWS_ACCESS_TOKEN set
          CLOUDFLARE_WORKER_PROVISION_TOKEN in bws
command:  cfops -wr whoami
stdout:   contains authenticated account info (from wrangler)
stderr:   (empty or wrangler status lines)
exit:     0
```

### long form is equivalent to short form

```
command:  cfops --wrangler whoami
behaviour: identical to cfops -wr whoami
```

### stdout from wrangler passes through

```
pre:      BWS_ACCESS_TOKEN set
command:  cfops -wr version
stdout:   contains wrangler version string
exit:     0
```

---

## `deploy`

### delegates to wrangler deploy

```
pre:      BWS_ACCESS_TOKEN set
          valid wrangler.toml in cwd
command:  cfops deploy
behaviour: equivalent to cfops -wr deploy
```

---

## `delete`

### delegates to wrangler delete

```
pre:      BWS_ACCESS_TOKEN set
          valid wrangler.toml in cwd
command:  cfops delete
behaviour: equivalent to cfops -wr delete
```

---

## `-tf` / `--terraform`

### passes args to terraform with all env vars mapped

```
pre:      BWS_ACCESS_TOKEN set
          CLOUDFLARE_DNS_CONFIG_TOKEN in bws
          CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID in bws
          CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY in bws
          CLOUDFLARE_ACCOUNT_ID in bws
command:  cfops -tf version
stdout:   contains terraform version string
exit:     0
```

### env var mapping

When `-tf` runs terraform, the subprocess receives:

| env var set for subprocess | sourced from bws secret |
|---------------------------|------------------------|
| `CLOUDFLARE_API_TOKEN` | `CLOUDFLARE_DNS_CONFIG_TOKEN` |
| `AWS_ACCESS_KEY_ID` | `CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID` |
| `AWS_SECRET_ACCESS_KEY` | `CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY` |
| `TF_VAR_account_id` | `CLOUDFLARE_ACCOUNT_ID` |

### long form is equivalent to short form

```
command:  cfops --terraform version
behaviour: identical to cfops -tf version
```

### stdout from terraform passes through

```
pre:      BWS_ACCESS_TOKEN set
command:  cfops -tf version
stdout:   contains terraform version string
stderr:   (empty)
exit:     0
```

### propagates terraform exit code

```
pre:      BWS_ACCESS_TOKEN set
command:  cfops -tf force-unlock --help  (or any tf command that exits non-zero)
exit:     matches terraform exit code
```

---

## `list`

### opens dashboard URL for active account

```
pre:      BWS_ACCESS_TOKEN set
          CLOUDFLARE_ACCOUNT_ID in bws
command:  cfops list
behaviour: opens https://dash.cloudflare.com/<CLOUDFLARE_ACCOUNT_ID>/workers-and-pages in browser
exit:     0
```
