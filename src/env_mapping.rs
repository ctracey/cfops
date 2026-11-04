use std::collections::HashMap;

pub struct EnvMapping {
    pub from: &'static str,
    pub to: &'static str,
}

pub const WRANGLER_MAPPINGS: &[EnvMapping] = &[EnvMapping {
    from: "CLOUDFLARE_WORKER_PROVISION_TOKEN",
    to: "CLOUDFLARE_API_TOKEN",
}];

pub const TERRAFORM_MAPPINGS: &[EnvMapping] = &[
    EnvMapping {
        from: "CLOUDFLARE_DNS_CONFIG_TOKEN",
        to: "CLOUDFLARE_API_TOKEN",
    },
    EnvMapping {
        from: "CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID",
        to: "AWS_ACCESS_KEY_ID",
    },
    EnvMapping {
        from: "CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY",
        to: "AWS_SECRET_ACCESS_KEY",
    },
    EnvMapping {
        from: "CLOUDFLARE_ACCOUNT_ID",
        to: "TF_VAR_account_id",
    },
];

pub fn apply_mappings(env: &mut HashMap<String, String>, mappings: &[EnvMapping]) {
    for mapping in mappings {
        if let Some(value) = env.get(mapping.from).cloned() {
            env.insert(mapping.to.to_string(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrangler_maps_provision_token_to_api_token() {
        let mut env = HashMap::new();
        env.insert(
            "CLOUDFLARE_WORKER_PROVISION_TOKEN".to_string(),
            "worker_token_abc".to_string(),
        );

        apply_mappings(&mut env, WRANGLER_MAPPINGS);

        assert_eq!(
            env.get("CLOUDFLARE_API_TOKEN"),
            Some(&"worker_token_abc".to_string())
        );
    }

    #[test]
    fn terraform_maps_dns_token_to_api_token() {
        let mut env = HashMap::new();
        env.insert(
            "CLOUDFLARE_DNS_CONFIG_TOKEN".to_string(),
            "dns_token_xyz".to_string(),
        );

        apply_mappings(&mut env, TERRAFORM_MAPPINGS);

        assert_eq!(
            env.get("CLOUDFLARE_API_TOKEN"),
            Some(&"dns_token_xyz".to_string())
        );
    }

    #[test]
    fn terraform_maps_r2_credentials_to_aws_vars() {
        let mut env = HashMap::new();
        env.insert(
            "CLOUDFLARE_R2_TERRAFORM_STATE_ACCESS_KEY_ID".to_string(),
            "key_id_123".to_string(),
        );
        env.insert(
            "CLOUDFLARE_R2_TERRAFORM_STATE_SECRET_ACCESS_KEY".to_string(),
            "secret_456".to_string(),
        );

        apply_mappings(&mut env, TERRAFORM_MAPPINGS);

        assert_eq!(
            env.get("AWS_ACCESS_KEY_ID"),
            Some(&"key_id_123".to_string())
        );
        assert_eq!(
            env.get("AWS_SECRET_ACCESS_KEY"),
            Some(&"secret_456".to_string())
        );
    }

    #[test]
    fn terraform_maps_account_id_to_tf_var() {
        let mut env = HashMap::new();
        env.insert(
            "CLOUDFLARE_ACCOUNT_ID".to_string(),
            "account_789".to_string(),
        );

        apply_mappings(&mut env, TERRAFORM_MAPPINGS);

        assert_eq!(
            env.get("TF_VAR_account_id"),
            Some(&"account_789".to_string())
        );
    }

    #[test]
    fn mapping_skipped_when_source_var_absent() {
        let mut env = HashMap::new();

        apply_mappings(&mut env, WRANGLER_MAPPINGS);

        assert_eq!(env.get("CLOUDFLARE_API_TOKEN"), None);
    }

    #[test]
    fn source_var_preserved_after_mapping() {
        let mut env = HashMap::new();
        env.insert(
            "CLOUDFLARE_WORKER_PROVISION_TOKEN".to_string(),
            "token".to_string(),
        );

        apply_mappings(&mut env, WRANGLER_MAPPINGS);

        assert!(env.contains_key("CLOUDFLARE_WORKER_PROVISION_TOKEN"));
    }

}
