// SPDX-License-Identifier: MIT
// Copyright (C) 2023-2025 Anya Project Contributors

//! Discrete Log Contracts (DLC) implementation
//!
//! DLCs enable conditional payments based on the outcome of future events,
//! using oracle signatures to determine the outcome without revealing the
//! contract details on the blockchain. This module provides functionality
//! for creating, managing, and executing DLCs while preserving privacy.

mod oracle;
mod contract;
mod adaptor;
mod cet;

pub use oracle::{Oracle, OracleEvent, OracleAttestation, OraclePublicKey};
pub use contract::{DlcContract, ContractInfo, ContractOutcome, ContractBuilder, ContractOutput};
pub use adaptor::{AdaptorSignature, AdaptorSigner};
pub use cet::{ContractExecutionTransaction, CetBuilder};

use std::sync::Arc;

use anyhow::{anyhow, Result};
use bitcoin::{OutPoint, Script, ScriptBuf, Transaction, TxOut, Address, Txid, Network};
use bitcoin::psbt::Psbt;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use secp256k1_zkp::{schnorr::Signature, XOnlyPublicKey};
use serde::{Serialize, Deserialize};

/// Status of a DLC contract
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContractStatus {
    /// Contract funding transaction is not yet confirmed
    Pending,
    
    /// Contract is active and awaiting outcome
    Active,
    
    /// Contract has been executed with a specific outcome
    Executed {
        /// The outcome that was attested
        outcome: String,
        
        /// Transaction ID of the executed CET
        txid: Txid,
    },
    
    /// Contract has been refunded
    Refunded {
        /// Transaction ID of the refund transaction
        txid: Txid,
    },
    
    /// Contract timeout has been reached but not yet refunded
    TimedOut,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_contract_status_serialization() {
        // Test that contract status can be serialized and deserialized
        let executed_status = ContractStatus::Executed {
            outcome: "sunny".to_string(),
            txid: Txid::all_zeros(),
        };
        
        let serialized = serde_json::to_string(&executed_status).unwrap();
        let deserialized: ContractStatus = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(executed_status, deserialized);
    }
}
