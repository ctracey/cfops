use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

use crate::bws;
use crate::credentials::BwsToken;

const ENVRC_FILE: &str = ".envrc";
const GITIGNORE_FILE: &str = ".gitignore";
const TERRAFORM_DIR: &str = "terraform";
const TF_BACKEND_FILE: &str = "terraform/backend.hcl";
const TF_LOCAL_DIR: &str = ".terraform";
const TF_STATE_GLOB: &str = "*.tfstate";
const TF_STATE_BACKUP_GLOB: &str = "*.tfstate.backup";
const TF_LOCK_FILE: &str = ".terraform.lock.hcl";
const NODE_MINIMUM_MAJOR_VERSION: u32 = 22;
const CLOUDFLARE_ACCOUNT_ID_ENV: &str = "CLOUDFLARE_ACCOUNT_ID";

const WRANGLER_FILE: &str = "wrangler.toml";
const WRANGLER_TEMPLATE: &str = include_str!("../templates/wrangler/static-worker/wrangler.toml");
const ENVRC_TEMPLATE: &str = include_str!("../templates/direnv/.envrc");
const TF_MAIN: &str = include_str!("../templates/terraform/main.tf");
const TF_OUTPUTS: &str = include_str!("../templates/terraform/outputs.tf");
const TF_BACKEND: &str = include_str!("../templates/terraform/backend.hcl");
const TF_VARIABLES: &str = include_str!("../templates/terraform/variables.tf");

pub fn run() -> Result<()> {
    let mut actions: Vec<String> = Vec::new();

    section("Account setup");
    let account_name = setup_envrc()?;
    if account_name.is_some() {
        actions.push("run 'direnv allow' to activate".to_string());
        if let Some(msg) = node_version_warning() {
            actions.push(msg);
        }
        actions.push("verify credentials: cfops -wr whoami — you should see a Keychain password prompt".to_string());
    }

    section("Worker setup");
    if confirm("Set up a worker? [y/N]: ")? {
        scaffold_wrangler()?;
    }

    section("DNS setup");
    if confirm("Set up DNS for a domain? [y/N]: ")? {
        scaffold_terraform(account_name.as_deref())?;
    }

    if !actions.is_empty() {
        println!("\n\x1b[2m────────────────────────────────────\x1b[0m");
        println!("Setup almost complete\n");
        println!("\x1b[1mSETUP ACTIONS:\x1b[0m");
        for action in &actions {
            println!("  \x1b[33m⚠  {action}\x1b[0m");
        }
    }

    println!("\n\x1b[1mNEXT STEPS:\x1b[0m");
    println!("  \x1b[36mSee the usage scenarios guide for worker and Terraform provisioning:\x1b[0m");
    println!("  \x1b[36mhttps://github.com/ctracey/cfops/blob/main/docs/usage-scenarios.md\x1b[0m");

    Ok(())
}

fn section(title: &str) {
    println!("\n\x1b[1m{title}\x1b[0m");
}

fn setup_envrc() -> Result<Option<String>> {
    if Path::new(ENVRC_FILE).exists() {
        println!(".envrc already exists — skipping");
        return Ok(None);
    }

    let account_name = prompt("Bitwarden machine account name: ")?;
    println!("Initialising account config...");
    write_envrc(&account_name)?;
    println!("\x1b[32m✓\x1b[0m .envrc created");

    ensure_gitignore_entries("direnv", &[ENVRC_FILE])?;

    Ok(Some(account_name))
}

fn scaffold_wrangler() -> Result<()> {
    if Path::new(WRANGLER_FILE).exists() {
        println!("wrangler.toml already exists — skipping");
        return Ok(());
    }

    let name = prompt("Worker name (e.g. mysite-static, mysite-host): ")?;

    let assets_input = prompt("Assets directory [./dist]: ")?;
    let assets_dir = if assets_input.is_empty() {
        "./dist".to_string()
    } else {
        assets_input
    };

    println!("Initialising worker config...");
    let content = WRANGLER_TEMPLATE
        .replace("<WORKER_NAME>", &name)
        .replace("<COMPATIBILITY_DATE>", &today_iso())
        .replace("<ASSETS_DIRECTORY>", &assets_dir);

    fs::write(WRANGLER_FILE, content)?;
    println!("\x1b[32m✓\x1b[0m wrangler.toml created");

    Ok(())
}

fn today_iso() -> String {
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86400;

    // Civil date from days since 1970-01-01 (Hinnant's algorithm)
    let z = days as i64 + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!("{y:04}-{m:02}-{d:02}")
}

fn scaffold_terraform(account_name: Option<&str>) -> Result<()> {
    if Path::new(TERRAFORM_DIR).exists() {
        println!("terraform/ already exists — skipping");
        return Ok(());
    }

    let domain = prompt("Domain (e.g. acme.com): ")?;

    let derived_label = derive_resource_label(&domain);
    let label_input = prompt(&format!("Terraform resource label [{derived_label}]: "))?;
    let resource_label = if label_input.is_empty() {
        derived_label
    } else {
        label_input
    };

    println!("Initialising DNS config...");
    let account_id = resolve_account_id(account_name)?;

    fs::create_dir_all(TERRAFORM_DIR)?;

    for (filename, template) in &[
        ("main.tf", TF_MAIN),
        ("outputs.tf", TF_OUTPUTS),
        ("backend.hcl", TF_BACKEND),
        ("variables.tf", TF_VARIABLES),
    ] {
        let content = template
            .replace("<DOMAIN>", &domain)
            .replace("<RESOURCE_LABEL>", &resource_label)
            .replace("<CLOUDFLARE_ACCOUNT_ID>", &account_id);
        fs::write(format!("{TERRAFORM_DIR}/{filename}"), content)?;
    }

    ensure_gitignore_entries("terraform", &[
        TF_BACKEND_FILE,
        TF_LOCAL_DIR,
        TF_STATE_GLOB,
        TF_STATE_BACKUP_GLOB,
        TF_LOCK_FILE,
    ])?;

    println!("\x1b[32m✓\x1b[0m terraform/ scaffolded");

    Ok(())
}

fn resolve_account_id(account_name: Option<&str>) -> Result<String> {
    if let Some(name) = account_name {
        if let Ok(id) = fetch_account_id_from_keychain(name) {
            println!("\x1b[32m✓\x1b[0m CLOUDFLARE_ACCOUNT_ID fetched from Bitwarden");
            return Ok(id);
        }
    }

    if let Some(id) = std::env::var(CLOUDFLARE_ACCOUNT_ID_ENV).ok().filter(|v| !v.is_empty()) {
        println!("  Using CLOUDFLARE_ACCOUNT_ID from env");
        return Ok(id);
    }

    if let Ok(token) = crate::credentials::load() {
        if let Ok(id) = bws::get_secret(&token, CLOUDFLARE_ACCOUNT_ID_ENV) {
            println!("\x1b[32m✓\x1b[0m CLOUDFLARE_ACCOUNT_ID fetched from Bitwarden");
            return Ok(id);
        }
    }

    prompt("Cloudflare account ID: ")
}

fn fetch_account_id_from_keychain(account_name: &str) -> Result<String> {
    let keychain_service = format!("bws_{account_name}");
    let output = Command::new("security")
        .args(["find-generic-password", "-s", &keychain_service, "-w"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("keychain entry '{keychain_service}' not found");
    }

    let token_value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    bws::get_secret(&BwsToken(token_value), CLOUDFLARE_ACCOUNT_ID_ENV)
}

fn derive_resource_label(domain: &str) -> String {
    domain.split('.').next().unwrap_or(domain).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_label_strips_tld() {
        assert_eq!(derive_resource_label("acme.com"), "acme");
    }

    #[test]
    fn derive_label_uses_first_segment_for_subdomain() {
        assert_eq!(derive_resource_label("sub.acme.com"), "sub");
    }

    #[test]
    fn derive_label_returns_input_when_no_dot() {
        assert_eq!(derive_resource_label("acme"), "acme");
    }
}

fn write_envrc(account_name: &str) -> Result<()> {
    let content = ENVRC_TEMPLATE.replace("<ACCOUNT_NAME>", account_name);
    fs::write(ENVRC_FILE, content)?;
    Ok(())
}

fn ensure_gitignore_entries(comment: &str, paths: &[&str]) -> Result<()> {
    let missing: Vec<&str> = paths.iter().copied().filter(|p| !gitignore_contains(p)).collect();
    if missing.is_empty() {
        return Ok(());
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(GITIGNORE_FILE)?;

    writeln!(file, "\n# {comment}")?;
    for path in &missing {
        writeln!(file, "{path}")?;
        println!("\x1b[32m✓\x1b[0m {path} added to .gitignore");
    }

    Ok(())
}

fn gitignore_contains(path: &str) -> bool {
    fs::read_to_string(GITIGNORE_FILE)
        .map(|contents| contents.lines().any(|line| line == path))
        .unwrap_or(false)
}

fn confirm(message: &str) -> Result<bool> {
    let input = prompt(message)?;
    Ok(matches!(input.to_lowercase().as_str(), "y" | "yes"))
}

fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn node_version_warning() -> Option<String> {
    let output = Command::new("node").arg("--version").output().ok()?;
    let version_str = String::from_utf8_lossy(&output.stdout);
    let version_str = version_str.trim().trim_start_matches('v');

    let major: u32 = version_str
        .split('.')
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    if major < NODE_MINIMUM_MAJOR_VERSION {
        Some(format!(
            "node v{version_str} is below the required v{NODE_MINIMUM_MAJOR_VERSION} — upgrade node before using wrangler"
        ))
    } else {
        None
    }
}
