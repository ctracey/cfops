use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use anyhow::Result;

const ENVRC_FILE: &str = ".envrc";
const GITIGNORE_FILE: &str = ".gitignore";
const TERRAFORM_DIR: &str = "terraform";
const NODE_MINIMUM_MAJOR_VERSION: u32 = 22;
const CLOUDFLARE_ACCOUNT_ID_ENV: &str = "CLOUDFLARE_ACCOUNT_ID";

const ENVRC_TEMPLATE: &str = include_str!("../templates/direnv/.envrc");
const TF_MAIN: &str = include_str!("../templates/terraform/main.tf");
const TF_OUTPUTS: &str = include_str!("../templates/terraform/outputs.tf");
const TF_BACKEND: &str = include_str!("../templates/terraform/backend.hcl");
const TF_VARIABLES: &str = include_str!("../templates/terraform/variables.tf");

pub fn run() -> Result<()> {
    setup_envrc()?;
    scaffold_terraform()?;
    Ok(())
}

fn setup_envrc() -> Result<()> {
    if Path::new(ENVRC_FILE).exists() {
        println!(".envrc already exists — skipping");
        return Ok(());
    }

    let account_name = prompt("Bitwarden machine account name: ")?;
    write_envrc(&account_name)?;
    println!("\x1b[32m✓\x1b[0m .envrc created");

    ensure_gitignore_entry()?;

    println!(
        "\x1b[34m→\x1b[0m ACTION: run 'direnv allow' to activate. \
        (HINT: this will attempt to read BWS_ACCESS_TOKEN from Keychain \
        — if configured you should see a password prompt)"
    );

    warn_if_node_too_old();

    println!("  HINT: test working with: cfops -wr whoami");

    Ok(())
}

fn scaffold_terraform() -> Result<()> {
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

    let account_id = match std::env::var(CLOUDFLARE_ACCOUNT_ID_ENV) {
        Ok(id) if !id.is_empty() => {
            println!("  Using CLOUDFLARE_ACCOUNT_ID from env");
            id
        }
        _ => prompt("Cloudflare account ID: ")?,
    };

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

    println!("\x1b[32m✓\x1b[0m terraform/ scaffolded");
    println!("\x1b[34m→\x1b[0m Next: cfops -tf init -backend-config=terraform/backend.hcl");

    Ok(())
}

fn derive_resource_label(domain: &str) -> String {
    domain.split('.').next().unwrap_or(domain).to_string()
}

fn write_envrc(account_name: &str) -> Result<()> {
    let content = ENVRC_TEMPLATE.replace("<ACCOUNT_NAME>", account_name);
    fs::write(ENVRC_FILE, content)?;
    Ok(())
}

fn ensure_gitignore_entry() -> Result<()> {
    if gitignore_contains_envrc() {
        return Ok(());
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(GITIGNORE_FILE)?;

    writeln!(file, "{ENVRC_FILE}")?;
    println!("\x1b[32m✓\x1b[0m .envrc added to .gitignore");

    Ok(())
}

fn gitignore_contains_envrc() -> bool {
    fs::read_to_string(GITIGNORE_FILE)
        .map(|contents| contents.lines().any(|line| line == ENVRC_FILE))
        .unwrap_or(false)
}

fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

fn warn_if_node_too_old() {
    let Ok(output) = Command::new("node").arg("--version").output() else {
        return;
    };

    let version_str = String::from_utf8_lossy(&output.stdout);
    let version_str = version_str.trim().trim_start_matches('v');

    let major: u32 = version_str
        .split('.')
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    if major < NODE_MINIMUM_MAJOR_VERSION {
        println!(
            "\x1b[34m→\x1b[0m ACTION: node version is v{version_str} \
            — wrangler requires node {NODE_MINIMUM_MAJOR_VERSION} or higher."
        );
    }
}
