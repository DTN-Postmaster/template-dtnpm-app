use clap::Parser;
use dtnpm_core::cli_security::{KeyStore, VerifyPolicy};
use hardy_bpa::bpa::BpaRegistration;
use hardy_bpv7::eid::Service;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use template_dtnpm_app::{AppConfig, AppService};
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "template-dtnpm-app",
    about = "DTN application and service daemon (template-dtnpm-app)",
    version
)]
struct Args {
    /// Port of the Hardy BPA gRPC server
    #[arg(short, long, default_value_t = 50051, env = "HARDY_GRPC_PORT")]
    port: u16,

    /// Optional path to a Unix Domain Socket for connecting to the BPA
    #[arg(long, env = "DTN_HDY_SOCKET")]
    socket: Option<String>,

    /// Path to file containing the bearer token (recommended for security)
    #[arg(long, env = "DTN_HDY_TOKEN_FILE")]
    token_file: Option<PathBuf>,

    /// Direct bearer token string (deprecated; prefer --token-file to avoid /proc leaks)
    #[arg(long, env = "DTN_HDY_TOKEN")]
    token: Option<String>,

    /// Optional configuration file path (YAML or TOML)
    #[arg(short, long, env = "DTN_APP_CONFIG")]
    config: Option<PathBuf>,

    /// Service endpoint to register (overrides config if specified)
    #[arg(short, long)]
    service: Option<String>,

    /// Path to BPSec keystore file
    #[arg(short, long)]
    keystore: Option<PathBuf>,

    /// BPSec verification policy: strict, warn, or ignore
    #[arg(long, default_value = "strict")]
    verify_policy: String,

    /// Explicitly allow unsigned/unauthenticated bundles under strict policy
    #[arg(long)]
    allow_unauthenticated: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn resolve_bearer_token(
    inline_token: Option<String>,
    token_file: Option<&Path>,
) -> anyhow::Result<Option<String>> {
    if let Some(path) = token_file {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read token file {}: {e}", path.display()))?;
        let trimmed = content.trim().to_string();
        if trimmed.is_empty() {
            anyhow::bail!("Token file {} is empty", path.display());
        }
        return Ok(Some(trimmed));
    }
    if let Some(tok) = inline_token.as_ref().filter(|t| !t.trim().is_empty()) {
        warn!("Passing token via CLI argument is deprecated for security; prefer --token-file");
        return Ok(Some(tok.clone()));
    }
    Ok(None)
}

fn load_config(path_opt: Option<&Path>) -> anyhow::Result<AppConfig> {
    let Some(path) = path_opt else {
        return Ok(AppConfig::default());
    };
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {e}", path.display()))?;

    let is_toml = path.extension().and_then(|s| s.to_str()) == Some("toml");
    let cfg: AppConfig = if is_toml {
        toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse TOML config {}: {e}", path.display()))?
    } else {
        yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse YAML config {}: {e}", path.display()))?
    };

    cfg.validate()
        .map_err(|e| anyhow::anyhow!("Invalid configuration: {e}"))?;
    Ok(cfg)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let env_filter = if args.verbose {
        "info,template_dtnpm_app=debug"
    } else {
        "info"
    };
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    info!("Starting template-dtnpm-app daemon");

    let mut config = load_config(args.config.as_deref())?;
    if let Some(svc) = args.service {
        config.service_name = svc;
    }
    if args.allow_unauthenticated {
        config.allow_unauthenticated = true;
    }

    let policy_str = if !args.verify_policy.is_empty() {
        args.verify_policy.as_str()
    } else {
        config.verify_policy.as_str()
    };
    let policy = match policy_str.to_ascii_lowercase().as_str() {
        "warn" => VerifyPolicy::Warn,
        "ignore" => VerifyPolicy::Ignore,
        _ => VerifyPolicy::Strict,
    };

    let keystore_path = args.keystore.or(config.keystore_path.clone());
    let keystore = if let Some(ref path) = keystore_path {
        KeyStore::load_file(path)
            .map_err(|e| anyhow::anyhow!("Failed to load keystore from {}: {e}", path.display()))?
    } else {
        KeyStore::empty()
    };

    let bearer_token = resolve_bearer_token(args.token, args.token_file.as_deref())?;
    let grpc_addr = format!("http://127.0.0.1:{}", args.port);

    info!(
        grpc_addr = %grpc_addr,
        socket = ?args.socket,
        service = %config.service_name,
        policy = ?policy,
        "Connecting to BPA daemon"
    );

    let (remote_bpa, _grpc_guard) =
        dtnpm_core::create_remote_bpa(&grpc_addr, bearer_token.as_deref(), args.socket.as_deref())
            .map_err(|e| anyhow::anyhow!("Failed to initialize BPA connection: {e}"))?;

    let service_id = if let Ok(num) = config.service_name.parse::<u32>() {
        Service::Ipn(num)
    } else {
        Service::Dtn(config.service_name.clone().into())
    };

    let app_service = Arc::new(AppService::new(config, policy, keystore));

    let registered_eid = remote_bpa
        .register_service(service_id, app_service.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to register service with BPA: {e}"))?;

    info!(eid = %registered_eid, "Service registration confirmed by BPA");
    info!("Running. Press Ctrl+C to terminate.");

    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, unregistering service...");

    if let Some(sink) = app_service.sink() {
        sink.unregister().await;
    }

    info!("template-dtnpm-app exited cleanly");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_bearer_token() {
        let dir = tempfile::tempdir().unwrap();
        let tok_path = dir.path().join("secret.token");
        std::fs::write(&tok_path, "  my-secret-token  \n").unwrap();

        let resolved = resolve_bearer_token(None, Some(&tok_path)).unwrap();
        assert_eq!(resolved.as_deref(), Some("my-secret-token"));

        let empty_path = dir.path().join("empty.token");
        std::fs::write(&empty_path, "   \n").unwrap();
        assert!(resolve_bearer_token(None, Some(&empty_path)).is_err());
    }
}
