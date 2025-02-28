// src/bitcoin/shadow.rs
//
// This module provides a shadow mode implementation that runs Rust
// implementation with logging for debugging and testing purposes.

use crate::bitcoin::interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, BitcoinTransaction,
    BitcoinAddress, AddressType, TransactionInput, TransactionOutput,
    BlockHeader, BitcoinImplementationType
};
use std::sync::{Arc, Mutex};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Shadow mode implementation that runs Rust implementation with logging
pub struct ShadowModeImplementation {
    /// The primary implementation to use for actual results
    primary_implementation: BitcoinImplementationType,
    /// The adapter for accessing implementations
    adapter: Arc<crate::bitcoin::adapter::BitcoinAdapter>,
    /// Log file for recording operations
    log_file: Mutex<Option<File>>,
    /// Whether to log all operations or only mismatches
    log_all: bool,
}

impl ShadowModeImplementation {
    /// Create a new shadow mode implementation
    pub fn new(
        adapter: Arc<crate::bitcoin::adapter::BitcoinAdapter>,
        primary_implementation: BitcoinImplementationType,
        log_file: Option<String>,
        log_all: bool,
    ) -> Self {
        let log_file_handle = log_file.map(|path| {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Could not open log file: {}", e);
                    OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("bitcoin_shadow.log")
                        .expect("Failed to open fallback log file")
                })
        });
        
        ShadowModeImplementation {
            primary_implementation,
            adapter,
            log_file: Mutex::new(log_file_handle),
            log_all,
        }
    }
    
    /// Log operation results
    fn log_operation(&self, operation: &str, result: &BitcoinResult<impl std::fmt::Debug>) {
        if let Ok(mut log_file) = self.log_file.lock() {
            if let Some(file) = log_file.as_mut() {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                let log_entry = format!(
                    "[{}] Operation: {}\n  Result: {:?}\n\n",
                    timestamp, operation, result
                );
                
                if let Err(e) = file.write_all(log_entry.as_bytes()) {
                    eprintln!("Warning: Failed to write to log file: {}", e);
                }
            }
        }
    }
    
    /// Execute an operation with logging
    fn execute_operation<T, F>(&self, operation: &str, f: F) -> BitcoinResult<T>
    where
        T: std::fmt::Debug,
        F: FnOnce(&dyn BitcoinInterface) -> BitcoinResult<T>,
    {
        let impl_ref = self.adapter.get_implementation()?;
        let result = f(impl_ref.as_ref());
        
        if self.log_all {
            self.log_operation(operation, &result);
        }
        
        result
    }
}

impl BitcoinInterface for ShadowModeImplementation {
    fn get_transaction(&self, txid: &str) -> BitcoinResult<BitcoinTransaction> {
        self.execute_operation(&format!("get_transaction({})", txid), |impl_ref| {
            impl_ref.get_transaction(txid)
        })
    }
    
    fn get_block(&self, hash: &str) -> BitcoinResult<Vec<BitcoinTransaction>> {
        self.execute_operation(&format!("get_block({})", hash), |impl_ref| {
            impl_ref.get_block(hash)
        })
    }
    
    fn get_block_height(&self) -> BitcoinResult<u32> {
        self.execute_operation("get_block_height()", |impl_ref| {
            impl_ref.get_block_height()
        })
    }
    
    fn generate_address(&self, address_type: AddressType) -> BitcoinResult<BitcoinAddress> {
        self.execute_operation(&format!("generate_address({:?})", address_type), |impl_ref| {
            impl_ref.generate_address(address_type)
        })
    }
    
    fn create_transaction(
        &self,
        outputs: Vec<(String, u64)>,
        fee_rate: u64,
    ) -> BitcoinResult<BitcoinTransaction> {
        self.execute_operation(
            &format!("create_transaction({:?}, {})", outputs, fee_rate),
            |impl_ref| impl_ref.create_transaction(outputs.clone(), fee_rate),
        )
    }
    
    fn broadcast_transaction(&self, transaction: &BitcoinTransaction) -> BitcoinResult<String> {
        self.execute_operation(
            &format!("broadcast_transaction({:?})", transaction.txid),
            |impl_ref| impl_ref.broadcast_transaction(transaction),
        )
    }
    
    fn get_balance(&self) -> BitcoinResult<u64> {
        self.execute_operation("get_balance()", |impl_ref| {
            impl_ref.get_balance()
        })
    }
    
    fn estimate_fee(&self, target_blocks: u8) -> BitcoinResult<u64> {
        self.execute_operation(&format!("estimate_fee({})", target_blocks), |impl_ref| {
            impl_ref.estimate_fee(target_blocks)
        })
    }
    
    fn implementation_type(&self) -> BitcoinImplementationType {
        self.primary_implementation
    }
}

/// Create a shadow mode implementation
pub fn create_shadow_mode_implementation(
    config: &crate::config::Config,
    primary_implementation: BitcoinImplementationType,
    log_file: Option<String>,
    log_all: bool,
) -> BitcoinResult<Arc<dyn BitcoinInterface>> {
    let adapter = Arc::new(crate::bitcoin::adapter::BitcoinAdapter::new(Arc::new(config.clone())));
    
    // Initialize the adapter
    adapter.initialize().map_err(|e| {
        BitcoinError::ImplementationError(format!("Failed to initialize adapter: {:?}", e))
    })?;
    
    let shadow_impl = ShadowModeImplementation::new(
        adapter,
        primary_implementation,
        log_file,
        log_all,
    );
    
    Ok(Arc::new(shadow_impl))
} 