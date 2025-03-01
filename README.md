# OPSource Bitcoin Implementation

A comprehensive Bitcoin platform built on Rust, focusing on decentralization, security, and privacy while adhering to core Bitcoin principles.

**Current Version**: 0.2.1  
**Last Updated**: March 1, 2025

## Core Principles

OPSource adheres to fundamental Bitcoin principles:

- **Decentralization**: Ensuring no single point of failure or control
- **Security**: Implementing rigorous security practices for all operations
- **Immutability**: Preserving the integrity and permanence of blockchain data
- **Privacy**: Enhancing user privacy through advanced cryptographic techniques
- **Transparency**: Providing clear visibility into all transaction processes

## Architecture

OPSource is built on a modular, hexagonal architecture with clean separation of concerns:

```
anya-core/
├── dependencies/
│   ├── anya-bitcoin/        # Core Bitcoin functionality
│   │   ├── src/
│   │   │   ├── wallet.rs    # Secure HD wallet implementation
│   │   │   ├── transaction.rs # Transaction handling
│   │   │   ├── taproot.rs   # Taproot implementation
│   │   │   ├── dlc/         # Discrete Log Contracts
│   │   │   ├── rgb/         # RGB asset protocol
│   │   │   ├── lightning/   # Lightning Network
│   │   │   ├── rsk/         # RSK sidechain integration
│   │   │   └── web5/        # Web5 and DID implementation
│   │   └── Cargo.toml       # Dependencies and configuration
│   └── anya-extensions/     # Additional functionality
└── src/
    ├── enterprise/          # Enterprise features
    └── extensions/          # Extension modules
```

## Key Features

### Core Bitcoin Functionality

- **Bitcoin Network Protocol**: Complete implementation using `rust-bitcoin`
- **HD Wallet Management**: BIP-32/39/44/84/86 compliant wallets using Bitcoin Dev Kit
- **Transaction Handling**: Advanced transaction construction and signing
- **Script Verification**: Complete Bitcoin Script interpreter
- **Mempool Management**: Transaction validation and propagation
- **P2P Networking**: Robust networking with peer discovery

### Advanced Bitcoin Features

- **Taproot Support**: Privacy-enhanced multisig that looks like single-sig
- **Discrete Log Contracts (DLC)**: Privacy-preserving oracle-based contracts
- **RGB Protocol**: Client-validated smart contracts and asset issuance on Bitcoin
- **Lightning Network**: Second-layer scaling solution for instant payments
- **RSK Integration**: Bitcoin-secured EVM-compatible smart contracts 
- **Stacks Integration**: Smart contracts secured by Bitcoin consensus

### DAO and Governance

- **Integrated DAO Platform**: Create and manage decentralized autonomous organizations
- **Multi-signature Governance**: Configure customizable voting thresholds and periods
- **Proposal System**: Generate, track, and vote on governance proposals
- **Metrics Dashboard**: Monitor DAO health and participation metrics
- **Extensible Rules Engine**: Create custom governance rules and constraints

### Security and Privacy

- **Memory Safety**: Rust's ownership model eliminates entire classes of vulnerabilities
- **Hardened Cryptography**: Modern, audited cryptographic libraries
- **Zero-Knowledge Proofs**: Privacy-enhancing cryptographic techniques
- **Coin Selection Privacy**: Algorithms to prevent transaction graph analysis
- **Tor Integration**: Network privacy via onion routing

### Web5 Integration

- **Decentralized Identifiers (DIDs)**: Self-sovereign identity management
- **Decentralized Web Nodes (DWNs)**: Personal data storage and management
- **Verifiable Credentials**: Privacy-preserving attestations
- **Handshake Support**: Decentralized DNS alternatives

### Machine Learning Features

- **Auto-configuration**: Optimizes ML settings based on hardware capabilities
- **Hardware Detection**: Identifies CPU, memory, and GPU resources
- **Framework Support**: TensorFlow and PyTorch integration
- **Performance Optimization**: Automatic batch size and parallelism configuration

## Getting Started

### Prerequisites

- Rust 1.77+
- Cargo
- OpenSSL development libraries

### Installation

```bash
# Clone the repository
git clone https://github.com/Anya-org/OPSource.git
cd OPSource

# Simple installation with default options
cargo run --bin installer -- install --yes

# Advanced installation with specific components
cargo run --bin installer -- install --opsource --setup-wallet --setup-dao --with-ml --ml-framework tensorflow --auto-config-ml

# Build the project
cargo build --release

# Run tests
cargo test --all-features
```

#### Installation Options

The unified installer supports various installation options:

```
USAGE:
    installer install [OPTIONS]

OPTIONS:
    --core-only                  Install core components only
    -y, --yes                    Skip confirmation prompts
    --with-python                Install Python dependencies
    --component <COMPONENT>      Specific component (bitcoin, web5, rgb, dlc, taproot, all)
    --opsource                   Install OPSource components
    --anya                       Install anya-core components
    --anya-modules <MODULES>     Anya modules (comma-separated: bitcoin,lightning,web5,extensions)
    --auto-config-anya           Auto-configure anya-core after installation
    --with-ml                    Install machine learning components
    --ml-framework <FRAMEWORK>   ML framework (tensorflow, pytorch, both)
    --auto-config-ml             Auto-configure ML based on machine specs
    --max-memory <MEMORY>        Maximum memory to use during installation (in MB)
    --setup-wallet               Setup wallet during installation
    --setup-dao                  Setup DAO during installation
```

### Testing

```bash
# Run all tests
cargo run --bin installer -- test

# Test specific components
cargo run --bin installer -- test --component wallet
cargo run --bin installer -- test --component dao
cargo run --bin installer -- test --component ml

# Generate JSON test report
cargo run --bin installer -- test --json
```

### Quick Start

```rust
use anya_bitcoin::{wallet, transaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create wallet configuration
    let config = wallet::WalletConfig {
        name: "my_wallet".to_string(),
        database_path: "./wallet.db".into(),
        network: bitcoin::Network::Testnet,
        electrum_url: "ssl://electrum.blockstream.info:60002".to_string(),
        password: None,
        mnemonic: None,
        use_taproot: true,
    };
    
    // Create wallet
    let wallet = wallet::BitcoinWallet::new(config).await?;
    
    // Get a new address
    let address = wallet.get_address(bdk::wallet::AddressIndex::New).await?;
    println!("New address: {}", address.address);
    
    Ok(())
}
```

## Example Use Cases

### Discrete Log Contracts (DLC)

```rust
// Create an oracle
let oracle = dlc::Oracle::new("Weather Oracle");
let oracle_pubkey = oracle.public_key();

// Create a DLC with specified outcomes
let outcomes = vec![
    ("sunny".to_string(), 1_000_000), // 1M sats if sunny
    ("rainy".to_string(), 500_000),   // 500K sats if rainy
];

// Create the contract
let contract = dlc::create_contract(
    wallet,
    outcomes,
    &oracle_pubkey.to_hex(),
).await?;

// Oracle attests to the outcome
let event = dlc::OracleEvent {
    id: "weather-2023-04-01".to_string(),
    outcome: "sunny".to_string(),
};
let attestation = oracle.attest(&event);

// Execute the contract based on attestation
let txid = dlc::execute_contract(
    wallet,
    &contract,
    attestation,
).await?;

// Check contract status
let status = dlc::get_contract_status(wallet, &contract).await?;
```

### DAO Creation and Management

```rust
// Create a new DAO configuration
let dao_config = dao::DAOConfig {
    name: "OPSource Governance DAO",
    governance_type: "multisig",
    voting_threshold: 0.66, // 66% required for proposal approval
    voting_period_days: 3,
    members: vec!["member1".to_string(), "member2".to_string()],
};

// Initialize the DAO
let dao = dao::create_dao(wallet, dao_config).await?;

// Create a new proposal
let proposal = dao::create_proposal(
    dao,
    "Increase Voting Threshold",
    "Proposal to increase voting threshold to 75%",
    json!({
        "action": "update_parameter",
        "parameter": "voting_threshold",
        "value": 0.75
    }),
).await?;

// Cast a vote
dao::vote(dao, &proposal.id, "member1", dao::Vote::Yes).await?;

// Check proposal status
let status = dao::get_proposal_status(dao, &proposal.id).await?;
```

### RGB Asset Issuance

```rust
// Issue a fungible asset on RGB
let asset = rgb::issue_asset(
    wallet,
    "My Token",
    "TKN",
    1_000_000, // 1M units
    "This is a sample token issued on RGB",
)?;
```

### RSK Smart Contract Interaction

```rust
// Interact with an RSK smart contract
let contract_address = "0x1234...";
let result = rsk::call_contract(
    wallet,
    contract_address,
    "balanceOf(address)",
    vec![my_address],
)?;
```

### Web5 DID Creation

```rust
// Create a decentralized identifier
let did = web5::create_did(wallet)?;
println!("Your DID: {}", did.to_string());

// Create a verifiable credential
let credential = web5::issue_credential(
    did,
    "EmailCredential",
    json!({ "email": "user@example.com" }),
)?;
```

## Security

OPSource implements state-of-the-art security practices:

- **Regular Security Audits**: Continuous vulnerability scanning
- **Dependency Management**: Automated updates for security patches
- **Static Analysis**: Comprehensive code quality checks
- **Fuzzing**: Automated testing against unexpected inputs
- **Peer Review**: All code changes undergo thorough review

See [SECURITY_FIXES.md](./SECURITY_FIXES.md) for security updates and vulnerability fixes.

## Migration from Python

We're in the process of migrating from Python to Rust. See [RUST_MIGRATION_PLAN.md](./RUST_MIGRATION_PLAN.md) for details on our phased approach to this transition.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Resources

- [Rust Bitcoin Book](https://rust-bitcoin.org/book/)
- [Bitcoin Dev Kit Documentation](https://bitcoindevkit.org/)
- [Lightning Dev Kit](https://lightningdevkit.org/)
- [RGB Protocol Documentation](https://docs.rgb.info/)
- [RSK Documentation](https://developers.rsk.co/)
- [Stacks Documentation](https://docs.stacks.co/)
- [TBD Web5](https://developer.tbd.website/projects/web5/)
