// Tests for the Rust Bitcoin implementation
// This module provides test utilities for the Bitcoin interface.

use crate::bitcoin::interface::{
    BitcoinInterface, BitcoinError, BitcoinResult, BitcoinTransaction,
    BitcoinAddress, AddressType, TransactionInput, TransactionOutput,
    BlockHeader, BitcoinImplementationType, create_bitcoin_interface
};
use crate::config::Config;

/// Run a test function against the Rust implementation
pub fn run_test<F, R>(test_fn: F) -> Result<R, String>
    where
    F: Fn(&dyn BitcoinInterface) -> BitcoinResult<R>,
    R: std::fmt::Debug,
{
    // Create a test configuration
    let mut config = Config::default();
    config.bitcoin_network = "regtest".to_string();
    config.bitcoin_rpc_url = "http://localhost:18443".to_string();
    
    // Test Rust implementation
    let rust_impl = create_bitcoin_interface(BitcoinImplementationType::Rust, &config);
    match test_fn(rust_impl.as_ref()) {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Rust implementation test failed: {:?}", e)),
    }
}

/// Test basic functionality of the Bitcoin interface
pub fn test_implementation(impl_ref: &dyn BitcoinInterface) -> Result<(), String> {
    // Test implementation type
    println!("Implementation type: {:?}", impl_ref.implementation_type());
    
    // Test address generation
    println!("Testing address generation...");
    let address = impl_ref.generate_address(AddressType::P2WPKH)
        .map_err(|e| format!("Failed to generate address: {:?}", e))?;
    println!("Generated address: {}", address.address);
    
    // Test fee estimation
    println!("Testing fee estimation...");
    let fee_rate = impl_ref.estimate_fee(6)
        .map_err(|e| format!("Failed to estimate fee: {:?}", e))?;
    println!("Estimated fee rate for 6 blocks: {} sat/vB", fee_rate);
    
    // Test balance retrieval
    println!("Testing balance retrieval...");
    let balance = impl_ref.get_balance()
        .map_err(|e| format!("Failed to get balance: {:?}", e))?;
    println!("Current balance: {} satoshis", balance);
    
    // Test block height retrieval
    println!("Testing block height retrieval...");
    let height = impl_ref.get_block_height()
        .map_err(|e| format!("Failed to get block height: {:?}", e))?;
    println!("Current block height: {}", height);
            
            Ok(())
}

/// Run a comprehensive test suite on the Bitcoin implementation
pub fn run_test_suite() -> Result<(), String> {
    println!("Running Bitcoin implementation test suite...");
    
    // Create a test configuration
    let mut config = Config::default();
    config.bitcoin_network = "regtest".to_string();
    config.bitcoin_rpc_url = "http://localhost:18443".to_string();
    
        println!("\nTesting Rust implementation:");
        let rust_impl = create_bitcoin_interface(BitcoinImplementationType::Rust, &config);
        test_implementation(rust_impl.as_ref()).map_err(|e| format!("Rust test error: {}", e))?;
    
    println!("\nAll tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_address_generation() {
        run_test(|impl_ref| {
            let address = impl_ref.generate_address(AddressType::P2WPKH)?;
            println!("Generated address: {}", address.address);
            Ok(())
        }).unwrap();
    }
    
    #[test]
    fn test_fee_estimation() {
        run_test(|impl_ref| {
            let fee_rate = impl_ref.estimate_fee(6)?;
            println!("Estimated fee rate: {} sat/vB", fee_rate);
    Ok(())
        }).unwrap();
    }
} 