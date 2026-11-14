use dtnpm_core::cli_security::{KeyStore, VerifyPolicy};
use template_dtnpm_app::{AppConfig, AppService};

#[tokio::test]
async fn test_app_config_validation() {
    let mut cfg = AppConfig::default();
    assert!(cfg.validate().is_ok());

    cfg.service_name = "   ".to_string();
    assert!(cfg.validate().is_err());

    cfg.service_name = "valid-service".to_string();
    cfg.verify_policy = "invalid-policy".to_string();
    assert!(cfg.validate().is_err());

    cfg.verify_policy = "strict".to_string();
    cfg.max_payload_size = 0;
    assert!(cfg.validate().is_err());
}

#[tokio::test]
async fn test_app_service_creation_and_policy() {
    let cfg = AppConfig {
        service_name: "test-echo".to_string(),
        max_payload_size: 1024 * 1024,
        verify_policy: "strict".to_string(),
        keystore_path: None,
        allow_unauthenticated: false,
    };
    let service = AppService::new(cfg, VerifyPolicy::Strict, KeyStore::empty());
    assert!(service.sink().is_none());

    // Process a dummy payload directly
    let id = hardy_bpv7::bundle::Id {
        source: "dtn://sender/".parse().unwrap(),
        timestamp: hardy_bpv7::creation_timestamp::CreationTimestamp::now(),
        fragment_info: None,
    };
    service.process_payload(&id, b"hello dtn").await;
}
