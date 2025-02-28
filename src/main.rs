// Main entry point for OPSource
// Allows testing of the Rust Bitcoin implementation

mod config;
mod bitcoin;

use std::env;
use std::sync::Arc;

fn main() {
    println!("OPSource Bitcoin Implementation Tester");
    println!("======================================");
    
    // Load configuration
    let mut config = config::Config::from_env();
    
    // Check for shadow mode
    let shadow_mode = env::var("SHADOW_MODE").map(|v| v.to_lowercase() == "true").unwrap_or(false);
    let log_file = env::var("LOG_FILE").ok();
    let log_all = env::var("LOG_ALL").map(|v| v.to_lowercase() == "true").unwrap_or(false);
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "test" => {
                println!("Running tests...");
                run_tests(shadow_mode, log_file, log_all);
                return;
            }
            "demo" => {
                println!("Running demo...");
                let bitcoin = bitcoin::create_bitcoin_interface(
                    bitcoin::interface::BitcoinImplementationType::Rust,
                    &config
                );
                if let Err(e) = run_demo(bitcoin.as_ref()) {
                    eprintln!("Error running demo: {:?}", e);
                }
                return;
            }
            "shadow" => {
                println!("Running in shadow mode with logging...");
                let log_file = log_file.or(Some("bitcoin_shadow.log".to_string()));
                run_tests(true, log_file, log_all);
                return;
            }
            _ => {
                print_usage();
                return;
            }
        }
    }
    
    // Default behavior: run demo with Rust implementation
    println!("Running demo with Rust implementation...");
    let bitcoin = bitcoin::create_bitcoin_interface(
        bitcoin::interface::BitcoinImplementationType::Rust,
        &config
    );
    if let Err(e) = run_demo(bitcoin.as_ref()) {
        eprintln!("Error running demo: {:?}", e);
    }
}

fn print_usage() {
    println!("Usage: opsource [COMMAND]");
    println!("Commands:");
    println!("  test   - Run tests on the Bitcoin implementation");
    println!("  demo   - Run a demo of the Bitcoin implementation");
    println!("  shadow - Run in shadow mode with logging");
    println!("");
    println!("Environment variables:");
    println!("  SHADOW_MODE     - Set to 'true' to enable shadow mode");
    println!("  LOG_FILE        - Path to log file for shadow mode");
    println!("  LOG_ALL         - Set to 'true' to log all operations in shadow mode");
}

fn run_demo(bitcoin: &dyn bitcoin::interface::BitcoinInterface) -> bitcoin::interface::BitcoinResult<()> {
    println!("Bitcoin implementation: {:?}", bitcoin.implementation_type());
    
    // Generate a new address
    println!("\nGenerating a new address...");
    let address = bitcoin.generate_address(bitcoin::interface::AddressType::P2WPKH)?;
    println!("Generated address: {}", address.address);
    
    // Get current balance
    println!("\nGetting current balance...");
    let balance = bitcoin.get_balance()?;
    println!("Current balance: {} satoshis", balance);
    
    // Estimate fee
    println!("\nEstimating fee for 6 block confirmation...");
    let fee_rate = bitcoin.estimate_fee(6)?;
    println!("Estimated fee rate: {} sat/vB", fee_rate);
    
    // Get current block height
    println!("\nGetting current block height...");
    let height = bitcoin.get_block_height()?;
    println!("Current block height: {}", height);
    
    // Create a transaction (this will fail if there are no funds)
    if balance > 0 {
        println!("\nCreating a transaction...");
        let recipient = "tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string();
        let amount = 10000; // 0.0001 BTC
        
        match bitcoin.create_transaction(vec![(recipient.clone(), amount)], fee_rate) {
            Ok(tx) => {
                println!("Transaction created successfully:");
                println!("  TXID: {}", tx.txid);
                println!("  Size: {} bytes", tx.size);
                println!("  Fee: {} satoshis", tx.fee.unwrap_or(0));
            }
            Err(e) => {
                println!("Failed to create transaction: {:?}", e);
            }
        }
    } else {
        println!("\nSkipping transaction creation (no funds available)");
    }
    
    Ok(())
}

fn run_tests(shadow_mode: bool, log_file: Option<String>, log_all: bool) {
    let config = config::Config::default();
    
    if shadow_mode {
        println!("Running tests in shadow mode...");
        
        // Create shadow mode implementation
        let result = bitcoin::create_bitcoin_interface_shadow_mode(
            &config,
            bitcoin::interface::BitcoinImplementationType::Rust,
            log_file,
            log_all
        );
        
        match result {
            Ok(bitcoin) => {
                if let Err(e) = run_tests_with_interface(bitcoin.as_ref()) {
                    eprintln!("Error running tests: {:?}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create shadow mode implementation: {:?}", e);
            }
        }
    } else {
        println!("Running tests with Rust implementation...");
        
        // Create Rust implementation
        let bitcoin = bitcoin::create_bitcoin_interface(
            bitcoin::interface::BitcoinImplementationType::Rust,
            &config
        );
        
        if let Err(e) = run_tests_with_interface(bitcoin.as_ref()) {
            eprintln!("Error running tests: {:?}", e);
        }
    }
}

fn run_tests_with_interface(bitcoin: &dyn bitcoin::interface::BitcoinInterface) -> bitcoin::interface::BitcoinResult<()> {
    println!("Testing implementation: {:?}", bitcoin.implementation_type());
    
    // Test address generation
    println!("Testing address generation...");
    let address = bitcoin.generate_address(bitcoin::interface::AddressType::P2WPKH)?;
    println!("Generated address: {}", address.address);
    
    // Test fee estimation
    println!("Testing fee estimation...");
    let fee_rate = bitcoin.estimate_fee(6)?;
    println!("Estimated fee rate: {} sat/vB", fee_rate);
    
    // Test balance retrieval
    println!("Testing balance retrieval...");
    let balance = bitcoin.get_balance()?;
    println!("Current balance: {} satoshis", balance);
    
    // Test block height retrieval
    println!("Testing block height retrieval...");
    let height = bitcoin.get_block_height()?;
    println!("Current block height: {}", height);
    
    println!("All tests passed!");
    Ok(())
} 