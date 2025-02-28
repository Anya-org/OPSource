# OPSource Bitcoin Implementation

A flexible, robust implementation of Bitcoin functionality with dual-support for Python and Rust, designed for smooth migration between implementations while maintaining a consistent interface.

**Current Version**: 0.1.0  
**Last Updated**: March 1, 2025

## Overview

OPSource provides a complete Bitcoin integration framework with the following key features:

- **Dual Implementation**: Side-by-side support for both Python (python-bitcoinlib) and Rust (rust-bitcoin/BDK) implementations
- **Common Interface**: Consistent API across implementations enabling seamless transitions
- **Migration Path**: Structured approach for transitioning from Python to Rust
- **Flexible Configuration**: Runtime selection of implementation based on configuration or feature flags
- **Comprehensive Testing**: Identical behavior validation across implementations

## Architecture

This project follows hexagonal architecture principles, with a clear separation between:

1. **Core Domain Interface** (`src/bitcoin/interface.rs`): Defines the contract that all implementations must fulfill
2. **Implementations**:
   - Python Implementation (`src/bitcoin/python.py`): Using python-bitcoinlib
   - Rust Implementation (`src/bitcoin/rust.rs`): Using rust-bitcoin and BDK
3. **Adapter Module** (`src/bitcoin/mod.rs`): Exposes a consistent interface and handles implementation selection
4. **Configuration** (`src/config.rs`): Manages environment settings and implementation preferences
5. **Testing Framework** (`src/bitcoin/test.rs`): Validates identical behavior across implementations

```
src/
├── bitcoin/
│   ├── interface.rs  # Common interface definition
│   ├── mod.rs        # Module and factory implementations
│   ├── python.py     # Python implementation
│   ├── rust.rs       # Rust implementation
│   └── test.rs       # Test framework
├── config.rs         # Configuration management
├── lib.rs            # Library exports
└── main.rs           # Test driver
```

## Getting Started

### Prerequisites

- Rust (1.60+)
- Python 3.8+
- python-bitcoinlib (`pip install python-bitcoinlib`)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/opsource.git
cd opsource

# Build with both implementations enabled
cargo build --features full
```

### Configuration

The implementation can be configured through environment variables:

```bash
# Select implementation (python or rust)
export USE_RUST_BITCOIN=1  # Use Rust implementation
export BITCOIN_NETWORK=testnet  # Network selection
```

Or programmatically:

```rust
let mut config = Config::default();
config.use_rust_bitcoin = true;
config.bitcoin_network = Some("testnet".to_string());
```

## Usage Examples

### Basic Usage

```rust
use opsource::{bitcoin, config};

// Initialize
let config = config::Config::default();
let bitcoin_interface = bitcoin::create_bitcoin_interface(
    bitcoin::interface::BitcoinImplementationType::Rust,
    &config
);

// Generate address
let address = bitcoin_interface.generate_address(
    bitcoin::interface::AddressType::P2WPKH
).unwrap();
println!("Generated address: {}", address.address);

// Check balance
let balance = bitcoin_interface.get_balance().unwrap();
println!("Wallet balance: {} satoshis", balance);

// Create transaction
let tx = bitcoin_interface.create_transaction(
    vec![("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx".to_string(), 10000)],
    5 // fee rate in sat/vB
).unwrap();
println!("Created transaction: {}", tx.txid);
```

### Testing Both Implementations

Use the provided script to test both implementations:

```bash
# Run tests on both implementations
./scripts/test_implementations.sh
```

## Migration Strategy

This project implements a controlled migration strategy from Python to Rust:

1. **Phase 1** (Complete): Dual implementation with Python as default
   - Common interface definition
   - Basic functionality in both implementations
   - Test framework to validate behavior

2. **Phase 2** (In Progress): Enhanced Rust implementation
   - Complete BDK wallet integration
   - Comprehensive error handling
   - Performance optimization

3. **Phase 3** (Upcoming): Rust as default implementation
   - Switch default to Rust implementation
   - Python as fallback only
   - Additional Bitcoin features (Lightning, etc.)

4. **Phase 4** (Future): Python removal
   - Complete removal of Python implementation
   - Full Rust implementation

## Implementation Details

### Python Implementation

- Uses python-bitcoinlib for core functionality
- Connects to Bitcoin network via RPC
- Provides all interface functionality with consistent error handling

### Rust Implementation

- Uses rust-bitcoin and BDK (Bitcoin Dev Kit)
- Wallet functionality via BDK with descriptor-based wallet support
- Electrum server connection for blockchain data
- Support for SegWit addresses and transactions

## Testing

The project includes a comprehensive test suite that ensures both implementations provide identical behavior:

```bash
# Run specific tests
cargo test

# Run the test suite comparing both implementations
cargo run -- test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Bitcoin Dev Kit (BDK) team
- python-bitcoinlib maintainers
- Bitcoin Core developers

# Bitcoin Code Migration

This repository contains scripts and documentation for migrating the Bitcoin implementation from the OPSource research project to the anya-core production codebase.

## Overview

The OPSource project served as a research initiative for exploring Bitcoin integration possibilities, while anya-core is the production-ready implementation. This migration ensures that all Bitcoin functionality follows the Bitcoin Development Framework v2.5 requirements while maintaining a Rust-only implementation.

## Files

- `migrate_bitcoin_code.ps1`: PowerShell script to perform the migration
- `test_bitcoin_migration.ps1`: PowerShell script to test the migration
- `BITCOIN_MIGRATION.md`: Detailed documentation of the migration process and module structure
- `README.md`: This file

## Migration Process

The migration process involves:

1. Creating a backup of the existing anya-core Bitcoin module
2. Copying and adapting files from OPSource to anya-core
3. Restructuring the code to fit the anya-core architecture
4. Updating import paths and adding appropriate headers

## Usage

### Prerequisites

- PowerShell 7.0 or higher
- Rust and Cargo (for compilation testing)

### Running the Migration

1. Ensure both OPSource and anya-core repositories are available
2. Run the migration script:

```powershell
.\migrate_bitcoin_code.ps1
```

3. Test the migration:

```powershell
.\test_bitcoin_migration.ps1
```

## Module Structure

The Bitcoin module in anya-core follows a hexagonal architecture as required by the Bitcoin Development Framework:

```
bitcoin/
├── adapters/           # Adapters for external interfaces
├── cross_chain/        # Cross-chain functionality
├── dlc/                # Discrete Log Contracts implementation
├── interface/          # Core interfaces
├── layer2/             # Layer 2 solutions
├── sidechains/         # Sidechain implementations
├── taproot/            # Taproot asset implementation
├── wallet/             # Wallet functionality
└── mod.rs              # Main Bitcoin module
```

## Compliance

The migrated code ensures compliance with:

- BIP 341/342 (Taproot)
- BIP 174 (PSBT)
- Miniscript Support
- Testnet Validation

## Further Information

For detailed information about the migration process and module structure, see [BITCOIN_MIGRATION.md](BITCOIN_MIGRATION.md).
