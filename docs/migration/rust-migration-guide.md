# Rust Migration Guide

This guide provides detailed information on migrating from the Python implementation to Rust for the OPSource project.

## Table of Contents

1. [Overview](#overview)
2. [Migration Strategy](#migration-strategy)
3. [Component Migration Status](#component-migration-status)
4. [API Compatibility](#api-compatibility)
5. [Development Workflow](#development-workflow)
6. [Testing](#testing)
7. [Upgrading Existing Applications](#upgrading-existing-applications)
8. [FAQ](#faq)

## Overview

The OPSource project has undergone a significant transformation, migrating core functionality from Python to Rust. This migration aims to:

- Improve performance and memory usage
- Enhance security through Rust's memory safety guarantees
- Provide better concurrency support
- Create a more robust and maintainable codebase
- Enable more native integrations with Bitcoin and Lightning libraries

## Migration Strategy

Our migration strategy followed these principles:

1. **Phase 1**: Core Bitcoin functionality (wallet, transactions, PSBT)
2. **Phase 2**: DLC implementation and API server
3. **Phase 3**: Advanced features (Lightning, RGB/Stacks, RSK)

For each component, we:
1. Created a Rust implementation with full test coverage
2. Ran parallel implementations (Python and Rust) to ensure functionality
3. Validated outputs with regression tests
4. Migrated users to the Rust implementation
5. Deprecated and removed the Python implementation

## Component Migration Status

Refer to [migration-status.md](./migration-status.md) for the current status of each component.

## API Compatibility

The Rust implementation maintains API compatibility with the Python version where possible, but some changes were necessary to leverage Rust's type system and safety features.

### Key API Changes

#### Wallet API

**Python (Old)**
```python
def create_transaction(outputs, fee_rate):
    # Implementation
```

**Rust (New)**
```rust
pub fn create_transaction(&self, outputs: Vec<(String, u64)>, fee_rate: u64) -> BitcoinResult<BitcoinTransaction> {
    // Implementation
}
```

#### DLC API

**Python (Old)**
```python
def create_dlc(contract_params):
    # Implementation
```

**Rust (New)**
```rust
pub fn create_dlc(&self, contract_params: ContractParams) -> Result<Contract, DlcError> {
    // Implementation
}
```

### Migration Helpers

The `compat` module provides compatibility layers to ease the transition:

```rust
// Using compat module to bridge Python and Rust APIs
use anya_bitcoin::compat::python::WalletCompat;

// Example of using compatibility layer
let compat_wallet = WalletCompat::new(wallet);
let result = compat_wallet.create_transaction_legacy(outputs, fee_rate);
```

## Development Workflow

### Setting Up Development Environment

1. Install Rust (1.70.0 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository
   ```bash
   git clone https://github.com/Anya-org/anya-core.git
   cd anya-core
   ```

3. Build the project
   ```bash
   cargo build
   ```

### Common Development Tasks

- **Running tests**
  ```bash
  cargo test
  ```

- **Building with optimizations**
  ```bash
  cargo build --release
  ```

- **Formatting code**
  ```bash
  cargo fmt
  ```

## Testing

Testing is a crucial part of the migration process. We've implemented several testing strategies:

### Unit Tests

Each component has comprehensive unit tests covering all functionality.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        // Test implementation
    }
}
```

### Integration Tests

Integration tests verify that components work together correctly.

```bash
# Run all integration tests
cargo test --test '*_integration'
```

### Cross-Implementation Tests

These tests compare outputs between Python and Rust implementations to ensure compatibility.

```rust
#[test]
fn test_create_transaction_matches_python() {
    // Test implementation comparing Rust output to Python reference
}
```

## Upgrading Existing Applications

To upgrade existing Python applications to use the Rust implementation:

1. **Install the Rust components**
   ```bash
   ./target/release/installer install
   ```

2. **Update Python imports**
   ```python
   # Old import
   from anya.bitcoin import BitcoinWallet

   # New import
   from anya.bitcoin.rust_bridge import BitcoinWallet
   ```

3. **Test thoroughly**
   Run your application's test suite to verify everything works as expected.

## FAQ

### Why migrate from Python to Rust?

Rust offers significant advantages in performance, safety, and concurrency. These benefits are especially important for cryptocurrency applications where security and reliability are paramount.

### Will the Python API be maintained?

The Python API will be maintained through compatibility layers during the transition period, but we recommend migrating to the Rust API for all new development.

### How do I report issues with the migration?

Please submit issues on our GitHub repository with the "migration" label.

### Is the Rust implementation stable?

Yes, all migrated components have undergone rigorous testing and are considered stable for production use.

### What if I need functionality that hasn't been migrated yet?

For functionality that hasn't been migrated, you can continue to use the Python implementation in the interim.

### How do I contribute to the migration effort?

See our [CONTRIBUTING.md](../../CONTRIBUTING.md) guide for information on how to contribute.
