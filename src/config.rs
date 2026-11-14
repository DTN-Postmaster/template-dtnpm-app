use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AppConfig {
    /// Service name registered with the BPA (e.g. "echo", "my-service").
    pub service_name: String,

    /// Maximum incoming payload size in bytes (default: 16 MiB).
    pub max_payload_size: usize,

    /// Verification policy for inbound bundles ("strict", "warn", "ignore").
    pub verify_policy: String,

    /// Path to the optional BPSec keystore file.
    pub keystore_path: Option<PathBuf>,

    /// Allow unauthenticated/unsigned bundles explicitly (opt-in).
    pub allow_unauthenticated: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            service_name: "template-service".to_string(),
            max_payload_size: 16 * 1024 * 1024,
            verify_policy: "strict".to_string(),
            keystore_path: None,
            allow_unauthenticated: false,
        }
    }
}

impl AppConfig {
    /// Validate configuration invariants and consistency.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.service_name.trim().is_empty() {
            return Err(crate::Error::Config(
                "service_name cannot be empty".to_string(),
            ));
        }
        if self.max_payload_size == 0 {
            return Err(crate::Error::Config(
                "max_payload_size must be greater than zero".to_string(),
            ));
        }
        let policy = self.verify_policy.to_ascii_lowercase();
        if !matches!(policy.as_str(), "strict" | "warn" | "ignore") {
            return Err(crate::Error::Config(format!(
                "invalid verify_policy '{}', expected 'strict', 'warn', or 'ignore'",
                self.verify_policy
            )));
        }
        Ok(())
    }
}
