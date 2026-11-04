use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// ── helpers ──────────────────────────────────────────────────────────────────

fn cfops() -> Command {
    Command::cargo_bin("cfops").unwrap()
}

fn create_mock_binary(dir: &Path, name: &str, script: &str) {
    let path = dir.join(name);
    fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

fn prepend_to_path(dir: &Path) -> String {
    let current = std::env::var("PATH").unwrap_or_default();
    format!("{}:{current}", dir.display())
}

// ── -v ───────────────────────────────────────────────────────────────────────

#[test]
fn version_flag_prints_version() {
    cfops()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("cfops "));
}

#[test]
fn version_output_contains_semver() {
    cfops()
        .arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"cfops \d+\.\d+\.\d+").unwrap());
}

// ── -h / --help ───────────────────────────────────────────────────────────────

#[test]
fn help_short_flag() {
    cfops()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("cfops"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("deploy"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("-wr"))
        .stdout(predicate::str::contains("--wrangler"))
        .stdout(predicate::str::contains("-tf"))
        .stdout(predicate::str::contains("--terraform"));
}

#[test]
fn help_long_flag_matches_short() {
    let short = cfops().arg("-h").output().unwrap();
    let long = cfops().arg("--help").output().unwrap();

    assert_eq!(short.stdout, long.stdout);
}

// ── unknown command ───────────────────────────────────────────────────────────

#[test]
fn unknown_command_exits_nonzero() {
    cfops()
        .arg("foobar")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown command"))
        .stderr(predicate::str::contains("cfops -h"));
}

#[test]
fn no_args_exits_nonzero() {
    cfops()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown command"));
}

#[test]
fn unknown_command_produces_no_stdout() {
    cfops()
        .arg("foobar")
        .assert()
        .failure()
        .stdout(predicate::str::is_empty());
}

// ── init ──────────────────────────────────────────────────────────────────────

#[test]
fn init_creates_envrc_and_gitignore_entry() {
    let dir = TempDir::new().unwrap();

    cfops()
        .arg("init")
        .current_dir(dir.path())
        .write_stdin("myaccount\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(".envrc created"))
        .stdout(predicate::str::contains(".envrc added to .gitignore"))
        .stdout(predicate::str::contains("direnv allow"))
        .stdout(predicate::str::contains("cfops -wr whoami"));

    let envrc = fs::read_to_string(dir.path().join(".envrc")).unwrap();
    assert!(envrc.contains("BWS_PROFILE_NAME=bws_myaccount"));
    assert!(envrc.contains("security find-generic-password"));

    let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
    assert!(gitignore.lines().any(|l| l == ".envrc"));
}

#[test]
fn init_skips_when_envrc_exists() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".envrc"), "existing content").unwrap();

    cfops()
        .arg("init")
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"));

    let contents = fs::read_to_string(dir.path().join(".envrc")).unwrap();
    assert_eq!(contents, "existing content");
}

#[test]
fn init_does_not_duplicate_envrc_in_gitignore() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".gitignore"), ".envrc\n").unwrap();

    cfops()
        .arg("init")
        .current_dir(dir.path())
        .write_stdin("myaccount\n")
        .assert()
        .success();

    let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
    let count = gitignore.lines().filter(|l| *l == ".envrc").count();
    assert_eq!(count, 1);
}

#[test]
fn init_appends_envrc_to_existing_gitignore() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join(".gitignore"), "node_modules\ndist\n").unwrap();

    cfops()
        .arg("init")
        .current_dir(dir.path())
        .write_stdin("myaccount\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(".envrc added to .gitignore"));

    let gitignore = fs::read_to_string(dir.path().join(".gitignore")).unwrap();
    assert!(gitignore.contains("node_modules"));
    assert!(gitignore.lines().any(|l| l == ".envrc"));
}

// ── credential loading ────────────────────────────────────────────────────────

#[test]
fn missing_bws_token_exits_with_error() {
    let mock_dir = TempDir::new().unwrap();

    // mock direnv that outputs nothing (no token)
    create_mock_binary(mock_dir.path(), "direnv", "exit 0");

    cfops()
        .arg("-wr")
        .arg("whoami")
        .env("BWS_ACCESS_TOKEN", "")
        .env("PATH", prepend_to_path(mock_dir.path()))
        .assert()
        .failure()
        .stderr(predicate::str::contains("BWS_ACCESS_TOKEN could not be loaded"))
        .stderr(predicate::str::contains("BWS_PROFILE_NAME"))
        .stderr(predicate::str::contains("docs/usage.md"));
}

// ── -wr / --wrangler ──────────────────────────────────────────────────────────

#[test]
fn wrangler_receives_mapped_cloudflare_api_token() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[{"id":"1","key":"CLOUDFLARE_WORKER_PROVISION_TOKEN","value":"worker_token_123","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"}]'
fi
"#,
    );

    create_mock_binary(
        mock_dir.path(),
        "wrangler",
        r#"echo "CLOUDFLARE_API_TOKEN=$CLOUDFLARE_API_TOKEN""#,
    );

    cfops()
        .args(["-wr", "whoami"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", prepend_to_path(mock_dir.path()))
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "CLOUDFLARE_API_TOKEN=worker_token_123",
        ));
}

#[test]
fn wrangler_long_flag_behaves_identically_to_short() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[{"id":"1","key":"CLOUDFLARE_WORKER_PROVISION_TOKEN","value":"token_abc","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"}]'
fi
"#,
    );

    create_mock_binary(
        mock_dir.path(),
        "wrangler",
        r#"echo "CLOUDFLARE_API_TOKEN=$CLOUDFLARE_API_TOKEN""#,
    );

    let path = prepend_to_path(mock_dir.path());

    let short = cfops()
        .args(["-wr", "whoami"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", &path)
        .output()
        .unwrap();

    let long = cfops()
        .args(["--wrangler", "whoami"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", &path)
        .output()
        .unwrap();

    assert_eq!(short.stdout, long.stdout);
    assert_eq!(short.status, long.status);
}

#[test]
fn wrangler_propagates_exit_code() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[]'
fi
"#,
    );

    create_mock_binary(mock_dir.path(), "wrangler", "exit 42");

    let output = cfops()
        .args(["-wr", "deploy"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", prepend_to_path(mock_dir.path()))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(42));
}

// ── -tf / --terraform ─────────────────────────────────────────────────────────

#[test]
fn terraform_receives_all_mapped_env_vars() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[{"id":"1","key":"CLOUDFLARE_DNS_CONFIG_TOKEN","value":"dns_token_xyz","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"},{"id":"2","key":"CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID","value":"key_id_abc","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"},{"id":"3","key":"CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY","value":"secret_def","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"},{"id":"4","key":"CLOUDFLARE_ACCOUNT_ID","value":"account_ghi","organizationId":"","projectId":"","note":"","creationDate":"2024-01-01T00:00:00Z","revisionDate":"2024-01-01T00:00:00Z"}]'
fi
"#,
    );

    create_mock_binary(
        mock_dir.path(),
        "terraform",
        r#"
echo "CLOUDFLARE_API_TOKEN=$CLOUDFLARE_API_TOKEN"
echo "AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID"
echo "AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY"
echo "TF_VAR_account_id=$TF_VAR_account_id"
"#,
    );

    cfops()
        .args(["-tf", "plan"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", prepend_to_path(mock_dir.path()))
        .assert()
        .success()
        .stdout(predicate::str::contains("CLOUDFLARE_API_TOKEN=dns_token_xyz"))
        .stdout(predicate::str::contains("AWS_ACCESS_KEY_ID=key_id_abc"))
        .stdout(predicate::str::contains("AWS_SECRET_ACCESS_KEY=secret_def"))
        .stdout(predicate::str::contains("TF_VAR_account_id=account_ghi"));
}

#[test]
fn terraform_long_flag_behaves_identically_to_short() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[]'
fi
"#,
    );

    create_mock_binary(mock_dir.path(), "terraform", "echo ok");

    let path = prepend_to_path(mock_dir.path());

    let short = cfops()
        .args(["-tf", "version"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", &path)
        .output()
        .unwrap();

    let long = cfops()
        .args(["--terraform", "version"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", &path)
        .output()
        .unwrap();

    assert_eq!(short.stdout, long.stdout);
    assert_eq!(short.status, long.status);
}

#[test]
fn terraform_propagates_exit_code() {
    let mock_dir = TempDir::new().unwrap();

    create_mock_binary(
        mock_dir.path(),
        "bws",
        r#"
if [ "$1" = "secret" ] && [ "$2" = "list" ]; then
    echo '[]'
fi
"#,
    );

    create_mock_binary(mock_dir.path(), "terraform", "exit 2");

    let output = cfops()
        .args(["-tf", "plan"])
        .env("BWS_ACCESS_TOKEN", "fake_token")
        .env("PATH", prepend_to_path(mock_dir.path()))
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
}
