use crate::config::AppConfig;
use dtnpm_core::cli_security::{KeyStore, VerifyPolicy, VerifyResult, verify_bundle};
use hardy_bpa::async_trait;
use hardy_bpa::services::{Result as BpaResult, Service as BpaService, ServiceSink};
use hardy_bpv7::eid::Eid;
use tokio::sync::OnceCell;
use tracing::{debug, error, info, warn};

/// Main BPA Service implementation for `template-dtnpm-app`.
pub struct AppService {
    config: AppConfig,
    policy: VerifyPolicy,
    keystore: KeyStore,
    sink: OnceCell<Box<dyn ServiceSink>>,
}

impl AppService {
    /// Create a new application service instance with configured policy and keystore.
    pub fn new(config: AppConfig, policy: VerifyPolicy, keystore: KeyStore) -> Self {
        Self {
            config,
            policy,
            keystore,
            sink: OnceCell::new(),
        }
    }

    /// Access the registered BPA sink if available.
    pub fn sink(&self) -> Option<&dyn ServiceSink> {
        self.sink.get().map(|s| s.as_ref())
    }
}

#[async_trait]
impl BpaService for AppService {
    async fn on_register(&self, source: &Eid, sink: Box<dyn ServiceSink>) {
        info!(eid = %source, "Service registered successfully with BPA");
        let _ = self.sink.set(sink);
    }

    async fn on_unregister(&self) {
        warn!("Service unregistered from BPA daemon");
    }

    async fn on_deliver(
        &self,
        id: &hardy_bpv7::bundle::Id,
        _expiry: time::OffsetDateTime,
        total_len: u64,
        stream: &mut dyn hardy_bpa::stream::Receiver<hardy_bpa::stream::Segment>,
    ) -> BpaResult<()> {
        let max_size = self.config.max_payload_size as u64;
        if total_len > max_size {
            error!(
                bundle_id = %id.to_key(),
                total_len,
                max_size,
                "Inbound bundle rejected: payload exceeds configured maximum size"
            );
            return Err(hardy_bpa::services::Error::PayloadTooLarge {
                size: total_len,
                max: max_size,
            });
        }

        let raw_cbor = hardy_bpa::stream::buffer_stream(stream, total_len).await?;
        debug!(
            bundle_id = %id.to_key(),
            bytes = raw_cbor.len(),
            "Buffered inbound bundle payload"
        );

        // Verify BPSec signature if enabled
        if self.policy != VerifyPolicy::Ignore {
            let res = verify_bundle(&raw_cbor, &self.keystore);
            match res {
                VerifyResult::Valid => {
                    debug!(bundle_id = %id.to_key(), "BPSec signature valid");
                }
                VerifyResult::Invalid(reason) => {
                    warn!(
                        bundle_id = %id.to_key(),
                        %reason,
                        "BPSec signature invalid"
                    );
                    if self.policy == VerifyPolicy::Strict {
                        error!("Dropping bundle due to strict BPSec policy");
                        return Ok(());
                    }
                }
                VerifyResult::Unsigned => {
                    if self.policy == VerifyPolicy::Strict && !self.config.allow_unauthenticated {
                        error!(
                            bundle_id = %id.to_key(),
                            "Dropping unsigned bundle under strict policy (opt-in allow_unauthenticated is false)"
                        );
                        return Ok(());
                    }
                    debug!(bundle_id = %id.to_key(), "Processing unsigned bundle");
                }
            }
        }

        // Parse outer bundle and extract payload
        let parsed = match hardy_bpv7::parse::parse(raw_cbor) {
            Ok(p) => p,
            Err(e) => {
                warn!(error = %e, "Failed to parse BPv7 bundle structure");
                return Ok(());
            }
        };

        if let Some(payload) = parsed
            .bundle
            .blocks
            .get(&1)
            .and_then(|block| block.payload(&parsed.data))
        {
            self.process_payload(id, payload).await;
        }

        Ok(())
    }

    async fn on_status_notify(
        &self,
        _bundle_id: &hardy_bpv7::bundle::Id,
        _from: &Eid,
        _kind: hardy_bpa::services::StatusNotify,
        _reason: hardy_bpv7::status_report::ReasonCode,
        _timestamp: Option<time::OffsetDateTime>,
    ) {
        // Handle delivery status notifications if requested
    }
}

impl AppService {
    /// Process delivered payload bytes. Custom business logic goes here.
    pub async fn process_payload(&self, id: &hardy_bpv7::bundle::Id, payload: &[u8]) {
        info!(
            bundle_id = %id.to_key(),
            source = %id.source,
            len = payload.len(),
            "Successfully received payload"
        );
        // Custom application logic: echo, save, parse, trigger, etc.
    }
}
