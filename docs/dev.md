# Developer Guide

## Credential Flow

How a command like `cfops -wr whoami` flows through the code:

```
 User
  │  cfops -wr whoami
  ▼
main.rs         parse_args() → Command::Wrangler(["whoami"])
  │             run_wrangler()
  ▼
credentials.rs  load()
  │               check env for BWS_ACCESS_TOKEN
  │               if missing → run `direnv export bash`
  │                              ↓
  │                         macOS Keychain prompt (once per session)
  │                              ↓
  │                         BWS_ACCESS_TOKEN loaded
  ▼
bws.rs          fetch_env(token)
  │               runs `bws secret list --output json`
  │               bws fetches only Bitwarden secrets (not CI env)
  │               merges with current process env (PATH, HOME, etc.)
  │               result: HashMap<String, String>
  │
  │             apply_mappings(env, WRANGLER_MAPPINGS)
  │               CLOUDFLARE_WORKER_PROVISION_TOKEN → CLOUDFLARE_API_TOKEN
  ▼
bws.rs          run_with_mappings()
  │               spawns `wrangler whoami`
  │               child process receives mapped env only
  │               stdin/stdout/stderr inherited from terminal
  ▼
 wrangler       runs with CLOUDFLARE_API_TOKEN set
                env vars gone when process exits
```

**Source files:**

| File | Responsibility |
|------|---------------|
| `src/main.rs` | Entry point, command dispatch |
| `src/cli.rs` | Argument parsing, help text |
| `src/credentials.rs` | BWS token loading via env or direnv |
| `src/env_mapping.rs` | Secret-to-env-var mapping definitions and logic |
| `src/bws.rs` | bws subprocess, env capture, tool execution |
| `src/init.rs` | `init` command — writes `.envrc`, updates `.gitignore` |

---

## Setup

Requires the Rust toolchain. Install via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
rustc --version
cargo --version
```

No other dependencies are needed to build or test. Runtime dependencies (`bws`, `direnv`, `wrangler`, `terraform`) are only required when running the binary against a real Cloudflare environment — integration tests mock them.

## Run Tests

```bash
cargo test
```

Unit tests cover argument parsing, env var mapping, and direnv export parsing. Integration tests use mock binaries to verify the full credential injection and env mapping chain without real credentials.

To run a specific test by name:

```bash
cargo test <test_name>
```

## Run Dev Build

Build and run a debug binary directly:

```bash
cargo run -- <command> [args]
```

Examples:

```bash
cargo run -- -h
cargo run -- -v
cargo run -- init
```

For commands that invoke `bws`, `wrangler`, or `terraform`, navigate to a project directory with a valid `.envrc` first:

```bash
cd ~/path/to/cloudflare-project
cargo run --manifest-path ~/path/to/cfops/Cargo.toml -- -wr whoami
```

## Package Binary

Build an optimised release binary:

```bash
cargo build --release
```

Output: `target/release/cfops`

To install it locally to `~/.cargo/bin`:

```bash
cargo install --path .
```

## Release

Releases are created by pushing a version tag. Before tagging, bump the version in `Cargo.toml`:

```toml
[package]
version = "x.y.z"
```

Then build, commit, tag, and push:

```bash
cargo build --release
git add Cargo.toml Cargo.lock
git commit -m "Bump version to x.y.z"
git tag vx.y.z
git push && git push origin vx.y.z
```

The workflow at `.github/workflows/release.yml` triggers on the tag, builds the release binary, and uploads it as a GitHub release asset for mise/ubi to consume.
