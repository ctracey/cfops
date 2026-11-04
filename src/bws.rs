use std::collections::HashMap;
use std::process::{ExitStatus, Stdio};
use anyhow::{Context, Result};

use crate::credentials::BwsToken;
use crate::env_mapping::{apply_mappings, EnvMapping};

fn fetch_secrets(token: &BwsToken) -> Result<HashMap<String, String>> {
    let output = std::process::Command::new("bws")
        .args(["run", "--", "env"])
        .env("BWS_ACCESS_TOKEN", &token.0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to run bws")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("bws failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_env_lines(&stdout))
}

fn parse_env_lines(output: &str) -> HashMap<String, String> {
    output
        .lines()
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

fn extract_required_secrets(
    bws_env: HashMap<String, String>,
    mappings: &[EnvMapping],
) -> HashMap<String, String> {
    let required_keys: std::collections::HashSet<&str> =
        mappings.iter().map(|m| m.from).collect();

    bws_env
        .into_iter()
        .filter(|(key, _)| required_keys.contains(key.as_str()))
        .collect()
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
    let bws_env = fetch_secrets(token)?;
    let secrets = extract_required_secrets(bws_env, mappings);

    let mut env: HashMap<String, String> = std::env::vars().collect();
    env.extend(secrets);
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
