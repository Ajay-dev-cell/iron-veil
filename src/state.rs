use crate::audit::AuditLogger;
use crate::config::AppConfig;
use chrono::{DateTime, Utc};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub connection_id: usize,
    pub event_type: String,
    pub content: String,
    pub details: Option<serde_json::Value>,
}

/// Upstream health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub last_check: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub latency_ms: Option<u64>,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            healthy: true, // Assume healthy until proven otherwise
            last_check: None,
            last_error: None,
            consecutive_failures: 0,
            consecutive_successes: 0,
            latency_ms: None,
        }
    }
}

/// Database protocol type for upstream connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DbProtocol {
    Postgres,
    MySql,
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub config_path: Arc<String>,
    pub active_connections: Arc<AtomicUsize>,
    pub logs: Arc<RwLock<VecDeque<LogEntry>>>,
    pub upstream_healthy: Arc<AtomicBool>,
    pub health_status: Arc<RwLock<HealthStatus>>,
    pub metrics_handle: Option<Arc<PrometheusHandle>>,
    /// Upstream database host for scanning
    pub upstream_host: Arc<String>,
    /// Upstream database port for scanning
    pub upstream_port: u16,
    /// Database protocol (Postgres or MySQL)
    pub db_protocol: DbProtocol,
    /// Audit logger for security events
    pub audit_logger: Arc<AuditLogger>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        config_path: String,
        upstream_host: String,
        upstream_port: u16,
        db_protocol: DbProtocol,
    ) -> Self {
        // Create audit logger from config
        let audit_logger = config
            .audit
            .as_ref()
            .map(|cfg| {
                AuditLogger::new(crate::audit::AuditConfig {
                    enabled: cfg.enabled,
                    log_to_stdout: cfg.log_to_stdout,
                    log_file: cfg.log_file.clone(),
                    rotation_enabled: cfg.rotation_enabled,
                    max_file_size_bytes: cfg.max_file_size_bytes,
                    max_rotated_files: cfg.max_rotated_files,
                    events: cfg.events.iter().map(|e| match e {
                        crate::config::AuditEventType::AuthAttempt => crate::audit::AuditEventType::AuthAttempt,
                        crate::config::AuditEventType::ConfigChange => crate::audit::AuditEventType::ConfigChange,
                        crate::config::AuditEventType::RuleAdded => crate::audit::AuditEventType::RuleAdded,
                        crate::config::AuditEventType::RuleDeleted => crate::audit::AuditEventType::RuleDeleted,
                        crate::config::AuditEventType::RulesImported => crate::audit::AuditEventType::RulesImported,
                        crate::config::AuditEventType::ConfigReload => crate::audit::AuditEventType::ConfigReload,
                        crate::config::AuditEventType::DatabaseScan => crate::audit::AuditEventType::DatabaseScan,
                        crate::config::AuditEventType::SchemaQuery => crate::audit::AuditEventType::SchemaQuery,
                        crate::config::AuditEventType::ApiAccess => crate::audit::AuditEventType::ApiAccess,
                    }).collect(),
                })
            })
            .unwrap_or_else(|| AuditLogger::new(crate::audit::AuditConfig::default()));

        Self {
            config: Arc::new(RwLock::new(config)),
            config_path: Arc::new(config_path),
            active_connections: Arc::new(AtomicUsize::new(0)),
            logs: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            upstream_healthy: Arc::new(AtomicBool::new(true)),
            health_status: Arc::new(RwLock::new(HealthStatus::default())),
            metrics_handle: None,
            upstream_host: Arc::new(upstream_host),
            upstream_port,
            db_protocol,
            audit_logger: Arc::new(audit_logger),
        }
    }

    /// Create a new AppState with default upstream settings (for testing)
    #[cfg(test)]
    pub fn new_for_test(config: AppConfig, config_path: String) -> Self {
        Self::new(
            config,
            config_path,
            "localhost".to_string(),
            5432,
            DbProtocol::Postgres,
        )
    }

    pub fn with_metrics(mut self, handle: PrometheusHandle) -> Self {
        self.metrics_handle = Some(Arc::new(handle));
        self
    }

    /// Save current config to the config file
    pub async fn save_config(&self) -> Result<(), std::io::Error> {
        let config = self.config.read().await;
        let yaml = serde_yaml::to_string(&*config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&*self.config_path, yaml)
    }

    pub async fn add_log(&self, entry: LogEntry) {
        let mut logs = self.logs.write().await;
        if logs.len() >= 100 {
            logs.pop_back();
        }
        logs.push_front(entry);
    }

    /// Check if upstream is healthy (fast atomic check)
    #[allow(dead_code)]
    pub fn is_upstream_healthy(&self) -> bool {
        self.upstream_healthy.load(Ordering::Relaxed)
    }

    /// Update upstream health status
    pub async fn update_health_status(
        &self,
        healthy: bool,
        latency_ms: Option<u64>,
        error: Option<String>,
    ) {
        let mut status = self.health_status.write().await;

        status.last_check = Some(Utc::now());
        status.latency_ms = latency_ms;

        if healthy {
            status.consecutive_successes += 1;
            status.consecutive_failures = 0;
            status.last_error = None;
        } else {
            status.consecutive_failures += 1;
            status.consecutive_successes = 0;
            status.last_error = error;
        }

        // Read config thresholds
        let config = self.config.read().await;
        let health_config = config.health_check.as_ref();
        let unhealthy_threshold = health_config.map(|h| h.unhealthy_threshold).unwrap_or(3);
        let healthy_threshold = health_config.map(|h| h.healthy_threshold).unwrap_or(1);
        drop(config);

        // Update healthy status based on thresholds
        if status.consecutive_failures >= unhealthy_threshold {
            status.healthy = false;
            self.upstream_healthy.store(false, Ordering::Relaxed);
        } else if status.consecutive_successes >= healthy_threshold {
            status.healthy = true;
            self.upstream_healthy.store(true, Ordering::Relaxed);
        }
    }

    /// Reload configuration from disk
    /// Returns the number of rules in the new config, or an error
    pub async fn reload_config(&self) -> Result<usize, String> {
        let path = self.config_path.as_ref();

        // Load new config from file
        let new_config = AppConfig::load(path)
            .map_err(|e| format!("Failed to load config from {}: {}", path, e))?;

        let rules_count = new_config.rules.len();

        // Update the config
        {
            let mut config = self.config.write().await;
            *config = new_config;
        }

        tracing::info!(
            "Configuration reloaded from {}: {} rules",
            path,
            rules_count
        );
        Ok(rules_count)
    }
}
