use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use anyhow::Result;

const ENVRC_FILE: &str = ".envrc";
const GITIGNORE_FILE: &str = ".gitignore";
const NODE_MINIMUM_MAJOR_VERSION: u32 = 22;

pub fn run() -> Result<()> {
    if Path::new(ENVRC_FILE).exists() {
        println!(".envrc already exists — nothing to do.");
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

fn write_envrc(account_name: &str) -> Result<()> {
    let content = format!(
        "export BWS_PROFILE_NAME=bws_{account_name}\n\
        export BWS_ACCESS_TOKEN=$(security find-generic-password -s \"$BWS_PROFILE_NAME\" -w)\n"
    );
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
