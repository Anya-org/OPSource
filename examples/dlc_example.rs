// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! DLC Example: How to create and execute a Discrete Log Contract
//! 
//! This example demonstrates the creation, execution, and refund of a 
//! Discrete Log Contract (DLC) using the Anya Bitcoin library.

use anyhow::Result;
use anya_bitcoin::{
    wallet::{BitcoinWallet, WalletConfig},
    dlc::{
        create_contract,
        execute_contract,
        refund_contract,
        get_contract_status,
        ContractStatus,
        Oracle, 
        OracleEvent,
        OracleAttestation,
    },
};
use bitcoin::Network;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("DLC Example: Weather Bet");
    println!("========================");
    
    // Create a wallet for Alice
    let alice_wallet = create_wallet("alice", Network::Testnet).await?;
    
    // Create a wallet for Bob
    let bob_wallet = create_wallet("bob", Network::Testnet).await?;
    
    // Create an oracle
    let oracle = Oracle::new("Weather Oracle");
    let oracle_pubkey = oracle.public_key();
    
    println!("Oracle Public Key: {}", oracle_pubkey.to_hex());
    
    // Define the contract outcomes
    let outcomes = vec![
        ("sunny".to_string(), 1_000_000), // Alice gets 1M sats if sunny
        ("rainy".to_string(), 500_000),   // Bob gets 500K sats if rainy
    ];
    
    // Create the contract
    println!("\nCreating DLC contract...");
    let contract = create_contract(&alice_wallet, outcomes, &oracle_pubkey.to_hex()).await?;
    
    println!("Contract created with ID: {}", contract.id);
    println!("Funding TXID: {}", contract.funding_txid);
    
    // Check initial contract status
    let status = get_contract_status(&alice_wallet, &contract).await?;
    println!("Contract status: {:?}", status);
    
    // Oracle attests to the outcome (in this case, "sunny")
    println!("\nOracle attesting to outcome: sunny");
    let event = OracleEvent {
        id: format!("weather-{}", contract.id),
        outcome: "sunny".to_string(),
    };
    
    let attestation = oracle.attest(&event);
    
    // Execute the contract
    println!("Executing contract based on attestation...");
    let execution_txid = execute_contract(&alice_wallet, &contract, attestation).await?;
    
    println!("Contract executed with TXID: {}", execution_txid);
    
    // Check final contract status
    let status = get_contract_status(&alice_wallet, &contract).await?;
    println!("Contract status: {:?}", status);
    
    // Example of refunding the contract (would normally only be done after timeout)
    println!("\nExample: Refunding a contract after timeout");
    println!("(Note: In a real scenario, this would only work after the timeout block height)");
    
    // Create another contract for refund example
    let refund_contract = create_contract(&bob_wallet, outcomes.clone(), &oracle_pubkey.to_hex()).await?;
    
    // Try to refund the contract (would fail in real scenario if timeout not reached)
    match refund_contract(&bob_wallet, &refund_contract).await {
        Ok(txid) => println!("Contract refunded with TXID: {}", txid),
        Err(e) => println!("Refund failed (expected if timeout not reached): {}", e),
    }
    
    println!("\nDLC Example completed successfully!");
    Ok(())
}

/// Helper function to create a wallet
async fn create_wallet(name: &str, network: Network) -> Result<BitcoinWallet> {
    let wallet_dir = PathBuf::from("./wallets");
    std::fs::create_dir_all(&wallet_dir)?;
    
    let config = WalletConfig {
        name: name.to_string(),
        database_path: wallet_dir.join(format!("{}.db", name)),
        network,
        electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
        password: None,
        mnemonic: None,
        use_taproot: true,
    };
    
    let wallet = BitcoinWallet::new(config).await?;
    println!("Created wallet for {}: {}", name, wallet.get_address(None).await?.address);
    
    Ok(wallet)
}
