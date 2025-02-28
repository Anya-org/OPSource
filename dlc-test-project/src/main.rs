// DLC Test Project
// This is a standalone project to test the DLC implementation

mod dlc;

use anyhow::Result;
use dlc::{
    Oracle, 
    OracleEvent,
    OracleAttestation,
    OraclePublicKey,
    ContractStatus,
};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("DLC Test Project");
    println!("===============");
    
    // Create an oracle
    let oracle = Oracle::new("Test Oracle");
    let oracle_pubkey = oracle.public_key();
    
    println!("Oracle Public Key: {}", oracle_pubkey.to_hex());
    
    // Create an event
    let event = OracleEvent {
        id: "test-event-1".to_string(),
        outcome: "success".to_string(),
    };
    
    // Attest to the event
    let attestation = oracle.attest(&event);
    
    println!("Event ID: {}", attestation.event_id);
    println!("Outcome: {}", attestation.outcome);
    
    // Verify the attestation
    let verification_result = attestation.verify(&oracle_pubkey);
    println!("Verification result: {}", verification_result);
    
    // Test contract status
    let status = ContractStatus::Active;
    let status_json = serde_json::to_string_pretty(&status)?;
    println!("Contract Status JSON: {}", status_json);
    
    println!("DLC Test completed successfully!");
    Ok(())
}
