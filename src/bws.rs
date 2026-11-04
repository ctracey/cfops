use std::collections::HashMap;
use std::process::{ExitStatus, Stdio};
use anyhow::{Context, Result};
use serde::Deserialize;

use crate::credentials::BwsToken;
use crate::env_mapping::{apply_mappings, EnvMapping};

#[derive(Deserialize)]
struct BwsSecret {
    key: String,
    value: String,
}

fn fetch_secrets(token: &BwsToken) -> Result<HashMap<String, String>> {
    let output = std::process::Command::new("bws")
        .args(["secret", "list", "--output", "json"])
        .env("BWS_ACCESS_TOKEN", &token.0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to run bws")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("bws failed: {stderr}");
    }

    let secrets: Vec<BwsSecret> =
        serde_json::from_slice(&output.stdout).context("failed to parse bws secret list output")?;

    Ok(secrets.into_iter().map(|s| (s.key, s.value)).collect())
}

pub fn get_secret(token: &BwsToken, key: &str) -> Result<String> {
    fetch_secrets(token)?
        .remove(key)
        .ok_or_else(|| anyhow::anyhow!("secret '{key}' not found in bws"))
}

pub fn run_with_mappings(
    binary: &str,
    args: &[String],
    token: &BwsToken,
    mappings: &[EnvMapping],
) -> Result<ExitStatus> {
    let mut env: HashMap<String, String> = std::env::vars().collect();
    env.extend(fetch_secrets(token)?);
    apply_mappings(&mut env, mappings);

    std::process::Command::new(binary)
        .args(args)
        .env_clear()
        .envs(&env)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("failed to spawn {binary}"))?
        .wait()
        .with_context(|| format!("failed to wait for {binary}"))
}
