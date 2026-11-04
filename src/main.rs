mod bws;
mod cli;
mod credentials;
mod env_mapping;
mod init;

use std::process;
use anyhow::Result;

use cli::{help_text, parse_args, Command, VERSION};
use credentials::load;
use env_mapping::{TERRAFORM_MAPPINGS, WRANGLER_MAPPINGS};

const CLOUDFLARE_ACCOUNT_ID: &str = "CLOUDFLARE_ACCOUNT_ID";
const CLOUDFLARE_DASHBOARD_URL: &str = "https://dash.cloudflare.com";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let exit_code = match run(args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    };

    process::exit(exit_code);
}

fn run(args: Vec<String>) -> Result<i32> {
    let Some(command) = parse_args(&args) else {
        eprintln!("Unknown command. Run '\x1b[1mcfops -h\x1b[0m' for usage.");
        return Ok(1);
    };

    match command {
        Command::Help => {
            println!("{}", help_text());
            Ok(0)
        }
        Command::Version => {
            println!("cfops {VERSION}");
            Ok(0)
        }
        Command::Init => {
            init::run()?;
            Ok(0)
        }
        Command::List => {
            let token = load()?;
            let account_id = bws::get_secret(&token, CLOUDFLARE_ACCOUNT_ID)?;
            open_in_browser(&format!(
                "{CLOUDFLARE_DASHBOARD_URL}/{account_id}/workers-and-pages"
            ))?;
            Ok(0)
        }
        Command::Deploy => run_wrangler(&["deploy".to_string()]),
        Command::Delete => run_wrangler(&["delete".to_string()]),
        Command::Wrangler(wrangler_args) => run_wrangler(&wrangler_args),
        Command::Terraform(terraform_args) => run_terraform(&terraform_args),
    }
}

fn run_wrangler(args: &[String]) -> Result<i32> {
    let token = load()?;
    let status = bws::run_with_mappings("wrangler", args, &token, WRANGLER_MAPPINGS)?;
    Ok(status.code().unwrap_or(1))
}

fn run_terraform(args: &[String]) -> Result<i32> {
    let token = load()?;
    let status = bws::run_with_mappings("terraform", args, &token, TERRAFORM_MAPPINGS)?;
    Ok(status.code().unwrap_or(1))
}

fn open_in_browser(url: &str) -> Result<()> {
    std::process::Command::new("open")
        .arg(url)
        .status()?;
    Ok(())
}
