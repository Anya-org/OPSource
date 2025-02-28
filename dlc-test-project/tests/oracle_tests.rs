use dlc_test_project::dlc::{Oracle, OracleEvent, OraclePublicKey};

#[test]
fn test_oracle_creation() {
    // Create a new oracle
    let oracle = Oracle::new("Test Oracle");
    
    // Get the oracle's public key
    let pubkey = oracle.public_key();
    
    // Ensure we can convert to and from hex
    let pubkey_hex = pubkey.to_hex();
    assert!(!pubkey_hex.is_empty());
    
    // Test hex conversion
    let parsed_pubkey = OraclePublicKey::from_hex(&pubkey_hex).unwrap();
    assert_eq!(pubkey, parsed_pubkey);
}

#[test]
fn test_oracle_attestation() {
    // Create a new oracle
    let oracle = Oracle::new("Weather Oracle");
    let pubkey = oracle.public_key();
    
    // Create an event to attest
    let event = OracleEvent {
        id: "weather-2023-01-01".to_string(),
        outcome: "sunny".to_string(),
    };
    
    // Create an attestation
    let attestation = oracle.attest(&event);
    
    // Verify that the event details match
    assert_eq!(attestation.event_id, event.id);
    assert_eq!(attestation.outcome, event.outcome);
    
    // Verify the attestation with the oracle's public key
    let result = attestation.verify(&pubkey);
    assert!(result);
    
    // Try with an incorrect outcome
    let mut bad_attestation = attestation.clone();
    bad_attestation.outcome = "rainy".to_string();
    
    // This should fail verification
    let result = bad_attestation.verify(&pubkey);
    assert!(!result);
}
