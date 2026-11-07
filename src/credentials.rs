use std::collections::HashMap;
use anyhow::{bail, Result};

const BWS_ACCESS_TOKEN: &str = "BWS_ACCESS_TOKEN";

pub struct BwsToken(pub String);

pub fn load() -> Result<BwsToken> {
    if let Ok(token) = std::env::var(BWS_ACCESS_TOKEN) {
        if !token.is_empty() {
            return Ok(BwsToken(token));
        }
    }

    if let Ok(token) = load_from_direnv() {
        return Ok(BwsToken(token));
    }

    bail!(
        "\x1b[31m✗\x1b[0m BWS_ACCESS_TOKEN could not be loaded. \
        Check that the BWS_PROFILE_NAME in .envrc matches the Keychain item name.\n\
        \x1b[34m→\x1b[0m See docs/setup.md section 1.3 to set up Keychain access."
    )
}

fn load_from_direnv() -> Result<String> {
    let output = std::process::Command::new("direnv")
        .args(["export", "bash"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let vars = parse_bash_exports(&stdout);

    vars.get(BWS_ACCESS_TOKEN)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("BWS_ACCESS_TOKEN not found in direnv output"))
}

pub fn parse_bash_exports(output: &str) -> HashMap<String, String> {
    output
        .split(['\n', ';'])
        .filter_map(|segment| {
            let segment = segment.trim();
            let rest = segment.strip_prefix("export ")?;
            let (key, value) = rest.split_once('=')?;
            let value = value.trim_matches('\'').trim_matches('"');
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_quoted_exports() {
        let output = "export BWS_ACCESS_TOKEN='token123'\nexport BWS_PROFILE_NAME='bws_test'\n";
        let vars = parse_bash_exports(output);

        assert_eq!(vars.get("BWS_ACCESS_TOKEN"), Some(&"token123".to_string()));
        assert_eq!(vars.get("BWS_PROFILE_NAME"), Some(&"bws_test".to_string()));
    }

    #[test]
    fn parses_double_quoted_exports() {
        let output = "export BWS_ACCESS_TOKEN=\"token123\"\n";
        let vars = parse_bash_exports(output);

        assert_eq!(vars.get("BWS_ACCESS_TOKEN"), Some(&"token123".to_string()));
    }

    #[test]
    fn parses_semicolon_separated_exports() {
        let output = "export FOO='bar';export BAZ='qux';";
        let vars = parse_bash_exports(output);

        assert_eq!(vars.get("FOO"), Some(&"bar".to_string()));
        assert_eq!(vars.get("BAZ"), Some(&"qux".to_string()));
    }

    #[test]
    fn skips_non_export_lines() {
        let output = "export VALID='yes'\nsome random line\n";
        let vars = parse_bash_exports(output);

        assert_eq!(vars.len(), 1);
        assert!(vars.contains_key("VALID"));
    }
}
