pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Version,
    Init,
    List,
    Deploy,
    Delete,
    Wrangler(Vec<String>),
    Terraform(Vec<String>),
}

pub fn parse_args(args: &[String]) -> Option<Command> {
    match args.first().map(String::as_str) {
        Some("-h" | "--help") => Some(Command::Help),
        Some("-v" | "--version") => Some(Command::Version),
        Some("init") => Some(Command::Init),
        Some("list") => Some(Command::List),
        Some("deploy") => Some(Command::Deploy),
        Some("delete") => Some(Command::Delete),
        Some("-wr" | "--wrangler") => Some(Command::Wrangler(args[1..].to_vec())),
        Some("-tf" | "--terraform") => Some(Command::Terraform(args[1..].to_vec())),
        _ => None,
    }
}

pub fn help_text() -> String {
    [
        format!("\x1b[1mcfops {VERSION}\x1b[0m — Cloudflare operations CLI"),
        String::new(),
        "\x1b[1mUsage:\x1b[0m".to_string(),
        "  cfops <command> [args]".to_string(),
        String::new(),
        "\x1b[1mCommands:\x1b[0m".to_string(),
        "  init                     Initialise .envrc for the current project".to_string(),
        "  list                     Open Workers & Pages dashboard for the active account".to_string(),
        "  deploy                   Deploy the worker (shorthand for -wr deploy)".to_string(),
        "  delete                   Delete the worker (shorthand for -wr delete)".to_string(),
        "  -wr, --wrangler <args>   Run any Wrangler command with credentials injected".to_string(),
        "  -tf, --terraform <args>  Run any Terraform command with credentials injected".to_string(),
        "  -v                       Show version".to_string(),
        "  -h                       Show this help".to_string(),
        String::new(),
        "\x1b[1mExamples:\x1b[0m".to_string(),
        "  cfops deploy".to_string(),
        "  cfops delete".to_string(),
        "  cfops list".to_string(),
        "  cfops -wr deployments status".to_string(),
        "  cfops -tf plan".to_string(),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_help_short() {
        assert_eq!(parse_args(&["-h".to_string()]), Some(Command::Help));
    }

    #[test]
    fn parses_help_long() {
        assert_eq!(parse_args(&["--help".to_string()]), Some(Command::Help));
    }

    #[test]
    fn parses_version() {
        assert_eq!(parse_args(&["-v".to_string()]), Some(Command::Version));
    }

    #[test]
    fn parses_init() {
        assert_eq!(parse_args(&["init".to_string()]), Some(Command::Init));
    }

    #[test]
    fn parses_wrangler_short() {
        let args = vec!["-wr".to_string(), "whoami".to_string()];
        assert_eq!(
            parse_args(&args),
            Some(Command::Wrangler(vec!["whoami".to_string()]))
        );
    }

    #[test]
    fn parses_wrangler_long() {
        let args = vec!["--wrangler".to_string(), "whoami".to_string()];
        assert_eq!(
            parse_args(&args),
            Some(Command::Wrangler(vec!["whoami".to_string()]))
        );
    }

    #[test]
    fn parses_terraform_short() {
        let args = vec!["-tf".to_string(), "plan".to_string()];
        assert_eq!(
            parse_args(&args),
            Some(Command::Terraform(vec!["plan".to_string()]))
        );
    }

    #[test]
    fn parses_terraform_long() {
        let args = vec!["--terraform".to_string(), "plan".to_string()];
        assert_eq!(
            parse_args(&args),
            Some(Command::Terraform(vec!["plan".to_string()]))
        );
    }

    #[test]
    fn wrangler_captures_all_trailing_args() {
        let args = vec![
            "-wr".to_string(),
            "deployments".to_string(),
            "status".to_string(),
        ];
        assert_eq!(
            parse_args(&args),
            Some(Command::Wrangler(vec![
                "deployments".to_string(),
                "status".to_string()
            ]))
        );
    }

    #[test]
    fn unknown_command_returns_none() {
        assert_eq!(parse_args(&["foobar".to_string()]), None);
    }

    #[test]
    fn empty_args_returns_none() {
        assert_eq!(parse_args(&[]), None);
    }

    #[test]
    fn help_text_contains_all_commands() {
        let text = help_text();
        for expected in &["init", "deploy", "delete", "list", "-wr", "--wrangler", "-tf", "--terraform"] {
            assert!(text.contains(expected), "help text missing: {expected}");
        }
    }
}
