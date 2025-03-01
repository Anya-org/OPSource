// Web5 Bitcoin anchoring test with Read First Always principle
#[cfg(feature = "web5")]
mod web5_read_first_tests {
    use anya_core::dependencies::anya_bitcoin::web5::enhanced_dwn::{EnhancedDwn, EnhancedDwnOptions};
    use anya_core::dependencies::anya_bitcoin::web5::credential::CredentialManager;
    use anya_core::dependencies::anya_bitcoin::web5::did::DidManager;
    use anya_core::dependencies::anya_bitcoin::wallet::BitcoinWallet;
    use anya_core::dependencies::anya_bitcoin::wallet::WalletConfig;
    use anya_core::web5::ReadFirstMetrics;
    use bitcoin::Network;
    use bdk::wallet::AddressIndex;
    use tempfile::tempdir;
    use std::sync::Arc;
    use std::collections::HashMap;
    use serde_json::Value;

    /// Wrapper structure to track Read First Always compliance
    struct TrackedCredentialManager {
        inner: CredentialManager,
        metrics: ReadFirstMetrics,
    }
    
    impl TrackedCredentialManager {
        /// Create a new tracked credential manager
        pub fn new(
            did_manager: Arc<DidManager>,
            wallet: Arc<BitcoinWallet>,
            network: Network,
        ) -> Self {
            Self {
                inner: CredentialManager::new(did_manager, wallet, network),
                metrics: ReadFirstMetrics::new("credentials"),
            }
        }
        
        /// Issue a credential with tracking of Read First Always principle
        pub async fn issue_anchored_credential(
            &self,
            issuer: &str,
            subject: &str,
            type_: &str,
            claims: HashMap<String, Value>,
            valid_days: Option<u64>,
        ) -> anyhow::Result<VerifiableCredential> {
            // Start timing for metrics
            let start_time = std::time::Instant::now();
            
            // READ FIRST ALWAYS: First read state before making changes
            let existing_credentials = self.inner.find_credentials_for_subject(subject).await?;
            
            // Track the read operation
            self.metrics.increment_reads();
            
            // Log the read operation
            log::info!(
                "READ_FIRST_ALWAYS: Read existing credentials for subject before issuing. Subject: {}, Found: {}",
                subject, 
                existing_credentials.len()
            );
            
            // Proceed with issuing the credential
            let credential = self.inner.issue_credential(
                issuer,
                subject,
                type_,
                claims,
                valid_days,
            ).await?;
            
            // Track the write operation
            self.metrics.increment_writes();
            
            // Log the write operation
            log::info!(
                "READ_FIRST_ALWAYS: Issued credential after reading existing state. Elapsed: {:?}",
                start_time.elapsed()
            );
            
            // Anchor the credential to Bitcoin
            let anchored = self.inner.anchor_credential_to_bitcoin(&credential).await?;
            
            // Track the write operation for anchoring
            self.metrics.increment_writes();
            
            // Log the anchoring operation
            log::info!(
                "READ_FIRST_ALWAYS: Anchored credential to Bitcoin. TXID: {}, Elapsed: {:?}",
                anchored.anchoring_transaction_id(),
                start_time.elapsed()
            );
            
            Ok(anchored)
        }
        
        /// Verify a credential with Read First tracking
        pub async fn verify_credential(
            &self,
            credential: &VerifiableCredential,
        ) -> anyhow::Result<bool> {
            // Track the read operation
            self.metrics.increment_reads();
            
            // Log the verification operation
            log::info!(
                "READ_FIRST_ALWAYS: Verifying credential: {}",
                credential.id()
            );
            
            // Verify the credential
            let valid = self.inner.verify_credential(credential).await?;
            
            // Verify Bitcoin anchoring
            if valid && credential.has_bitcoin_anchoring() {
                let anchoring_valid = self.inner.verify_credential_anchoring(credential).await?;
                
                // Log the anchoring verification
                log::info!(
                    "READ_FIRST_ALWAYS: Verified Bitcoin anchoring for credential: {}, Valid: {}",
                    credential.id(),
                    anchoring_valid
                );
                
                return Ok(anchoring_valid);
            }
            
            Ok(valid)
        }
        
        /// Get metrics
        pub fn get_metrics(&self) -> &ReadFirstMetrics {
            &self.metrics
        }
    }

    #[tokio::test]
    async fn test_web5_credential_with_read_first_principle() -> anyhow::Result<()> {
        // Create a temporary directory for the test
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test_db.sqlite");
        
        // Initialize wallet with testnet configuration
        let wallet_config = WalletConfig {
            network: Network::Testnet,
            mnemonic: Some("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string()),
            passphrase: None,
            db_path: Some(db_path.to_str().unwrap().to_string()),
            ..Default::default()
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Create DID manager
        let did_manager = Arc::new(DidManager::new(wallet.clone(), Network::Testnet));
        
        // Create the tracked credential manager
        let tracked_manager = TrackedCredentialManager::new(
            did_manager.clone(),
            wallet.clone(),
            Network::Testnet,
        );
        
        // Create a test DID
        let issuer_did = did_manager.create_did("key").await?;
        let subject_did = did_manager.create_did("key").await?;
        
        // Prepare test claims
        let mut claims = HashMap::new();
        claims.insert("name".to_string(), Value::String("Test User".to_string()));
        claims.insert("email".to_string(), Value::String("test@example.com".to_string()));
        
        // Issue a credential with Read First tracking
        let credential = tracked_manager.issue_anchored_credential(
            &issuer_did.id,
            &subject_did.id,
            "TestCredential",
            claims,
            Some(365),
        ).await?;
        
        // Verify the credential with Read First tracking
        let valid = tracked_manager.verify_credential(&credential).await?;
        assert!(valid, "Credential should be valid");
        
        // Check metrics for Read First compliance
        let metrics = tracked_manager.get_metrics();
        assert!(metrics.reads() >= 1, "Should have performed at least 1 read");
        assert!(metrics.writes() >= 2, "Should have performed at least 2 writes for issuance and anchoring");
        assert_eq!(metrics.violations(), 0, "Should not have any violations");
        
        // Validate read-write ratio
        let ratio = metrics.read_write_ratio();
        assert!(ratio > 0.0, "Read-write ratio should be positive");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_read_first_multiple_operations() -> anyhow::Result<()> {
        // Create a temporary directory for the test
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test_multi_db.sqlite");
        
        // Initialize wallet with testnet configuration
        let wallet_config = WalletConfig {
            network: Network::Testnet,
            mnemonic: Some("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string()),
            passphrase: None,
            db_path: Some(db_path.to_str().unwrap().to_string()),
            ..Default::default()
        };
        
        let wallet = Arc::new(BitcoinWallet::new(wallet_config)?);
        
        // Setup enhanced DWN with Read First Always principle
        let dwn_options = EnhancedDwnOptions {
            network: Network::Testnet,
            storage_path: temp_dir.path().join("dwn_db").to_str().unwrap().to_string(),
            enable_bitcoin_anchoring: true,
            enforce_read_first_always: true,
        };
        
        let enhanced_dwn = EnhancedDwn::new(wallet.clone(), dwn_options)?;
        
        // Create DID manager
        let did_manager = Arc::new(DidManager::new(wallet.clone(), Network::Testnet));
        
        // Create the tracked credential manager
        let tracked_manager = TrackedCredentialManager::new(
            did_manager.clone(),
            wallet.clone(),
            Network::Testnet,
        );
        
        // Create test DIDs
        let issuer_did = did_manager.create_did("key").await?;
        
        // Issue multiple credentials to different subjects
        let subject_count = 5;
        let mut credentials = Vec::with_capacity(subject_count);
        
        for i in 0..subject_count {
            // Create a subject DID
            let subject_did = did_manager.create_did("key").await?;
            
            // Prepare claims
            let mut claims = HashMap::new();
            claims.insert("name".to_string(), Value::String(format!("Test User {}", i)));
            claims.insert("id".to_string(), Value::Number(i.into()));
            
            // Issue credential
            let credential = tracked_manager.issue_anchored_credential(
                &issuer_did.id,
                &subject_did.id,
                "TestCredential",
                claims,
                Some(365),
            ).await?;
            
            credentials.push(credential);
        }
        
        // Verify all credentials
        for credential in &credentials {
            let valid = tracked_manager.verify_credential(credential).await?;
            assert!(valid, "Credential should be valid");
            
            // Store the credential in DWN with Read First tracking
            let record_id = enhanced_dwn.store_credential(credential).await?;
            assert!(!record_id.is_empty(), "Record ID should not be empty");
            
            // Retrieve the credential to verify storage
            let retrieved = enhanced_dwn.get_credential(&record_id).await?;
            assert!(retrieved.is_some(), "Should retrieve the stored credential");
        }
        
        // Check metrics for Read First compliance in credential manager
        let cm_metrics = tracked_manager.get_metrics();
        assert!(cm_metrics.reads() >= subject_count, 
                "Should have performed at least one read per subject");
        assert!(cm_metrics.writes() >= subject_count * 2, 
                "Should have performed at least two writes per subject (issuance and anchoring)");
        assert_eq!(cm_metrics.violations(), 0, "Should not have any violations");
        
        // Check metrics for the enhanced DWN
        let dwn_metrics = enhanced_dwn.read_first_metrics();
        assert!(dwn_metrics.reads() >= subject_count * 2, 
                "DWN should have performed at least two reads per credential (store and retrieve)");
        assert!(dwn_metrics.writes() >= subject_count, 
                "DWN should have performed at least one write per credential");
        assert_eq!(dwn_metrics.violations(), 0, "DWN should not have any violations");
        
        // Validate overall read-write ratio
        let cm_ratio = cm_metrics.read_write_ratio();
        let dwn_ratio = dwn_metrics.read_write_ratio();
        
        assert!(cm_ratio >= 0.5, "Credential manager read-write ratio should be at least 0.5");
        assert!(dwn_ratio >= 1.0, "DWN read-write ratio should be at least 1.0");
        
        // Log metrics summary
        log::info!(
            "Read First Metrics - Credential Manager: Reads={}, Writes={}, Ratio={:.2}, Violations={}",
            cm_metrics.reads(),
            cm_metrics.writes(),
            cm_ratio,
            cm_metrics.violations()
        );
        
        log::info!(
            "Read First Metrics - Enhanced DWN: Reads={}, Writes={}, Ratio={:.2}, Violations={}",
            dwn_metrics.reads(),
            dwn_metrics.writes(),
            dwn_ratio,
            dwn_metrics.violations()
        );
        
        Ok(())
    }
}
