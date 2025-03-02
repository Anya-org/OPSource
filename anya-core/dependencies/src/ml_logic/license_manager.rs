use lightning_auth::LightningAuth;
use web5_auth::Web5Auth;
use tokio;
use async_trait;

pub struct LicenseManager {
    auth_provider: Box<dyn BlockchainAuth>,
    metrics_collector: ApiMetricsCollector,
}

#[async_trait]
pub trait BlockchainAuth: Send + Sync {
    async fn verify(&self, credentials: &AuthCredentials) -> Result<bool, AuthError>;
}

impl LicenseManager {
    pub fn new(
        auth_provider: Box<dyn BlockchainAuth>,
        metrics_collector: ApiMetricsCollector,
    ) -> Self {
        Self {
            auth_provider,
            metrics_collector,
        }
    }

    pub async fn validate_license(&self, license_key: &str) -> Result<bool, LicenseError> {
        let metrics_valid = self.metrics_collector.validate(license_key).await?;
        let auth_valid = self.auth_provider.verify(&self.credentials).await?;
        
        Ok(metrics_valid && auth_valid)
    }

    pub async fn track_usage(&self, license_key: &str) -> Result<UsageMetrics, MetricsError> {
        self.metrics_collector.collect_and_process(license_key).await
    }
}
