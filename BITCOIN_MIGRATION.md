# Bitcoin Module Migration

## Overview

This document outlines the migration of the Bitcoin implementation from the OPSource research project to the anya-core production codebase. The migration ensures that all Bitcoin functionality follows the Bitcoin Development Framework v2.5 requirements while maintaining a Rust-only implementation.

## Migration Process

The migration is handled by the `migrate_bitcoin_code.ps1` PowerShell script, which:

1. Creates a backup of the existing anya-core Bitcoin module
2. Copies and adapts files from OPSource to anya-core
3. Restructures the code to fit the anya-core architecture
4. Updates import paths and adds appropriate headers

## Module Structure

The Bitcoin module in anya-core follows a hexagonal architecture as required by the Bitcoin Development Framework:

```
bitcoin/
├── adapters/           # Adapters for external interfaces
│   ├── mod.rs          # Main adapter module
│   └── rust.rs         # Rust-specific adapter implementation
├── cross_chain/        # Cross-chain functionality
│   ├── mod.rs          # Main cross-chain module
│   └── rsk/            # RSK bridge implementation
├── dlc/                # Discrete Log Contracts implementation
│   ├── mod.rs          # Main DLC module
│   └── oracle.rs       # Oracle implementation for DLCs
├── interface/          # Core interfaces
│   └── mod.rs          # Bitcoin interface definitions
├── layer2/             # Layer 2 solutions
│   └── lightning.rs    # Lightning Network implementation
├── sidechains/         # Sidechain implementations
├── taproot/            # Taproot asset implementation
│   ├── mod.rs          # Main Taproot module
│   └── assets.rs       # Taproot asset creation and management
├── wallet/             # Wallet functionality
└── mod.rs              # Main Bitcoin module
```

## Key Components

### Bitcoin Interface

The Bitcoin interface (`interface/mod.rs`) defines the core abstractions for interacting with the Bitcoin network, including:

- Transaction creation and signing
- Block validation
- SPV proof verification
- Wallet management

### Adapters

The adapters (`adapters/mod.rs` and `adapters/rust.rs`) implement the Bitcoin interface using Rust libraries, providing:

- Network communication
- Transaction broadcasting
- Block header validation
- UTXO management

### DLC Implementation

The DLC module (`dlc/mod.rs`) implements privacy-preserving Discrete Log Contracts using non-interactive oracle patterns, following the requirements in the Bitcoin Development Framework v2.5.

### Cross-Chain Functionality

The cross-chain module (`cross_chain/mod.rs`) provides functionality for interacting with other blockchains, including:

- RSK bridge implementation
- Bitcoin SPV proof creation and verification
- Cross-chain transaction management

### Taproot Assets

The Taproot module (`taproot/mod.rs`) implements Taproot-enabled protocols for asset issuance and management, compatible with the requirements in the Bitcoin Development Framework v2.5.

## Compliance

The migrated code ensures compliance with:

- [x] BIP 341/342 (Taproot)
- [x] BIP 174 (PSBT)
- [x] Miniscript Support
- [x] Testnet Validation

## Next Steps

After running the migration script:

1. Review the migrated code for any issues
2. Run the test suite to ensure functionality
3. Update any dependencies as needed
4. Document any changes to the API

## Backup

A backup of the original anya-core Bitcoin module is created during the migration process and stored in the `bitcoin_backup_[timestamp]` directory. 