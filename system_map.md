# System Architecture Map

Last Updated: March 1, 2025

## Project Structure

```text
OPSource/
├── anya-core/               # Core Bitcoin implementation
│   ├── src/                 # Core source code
│   │   ├── bitcoin/         # Bitcoin protocol integration
│   │   ├── web5/            # Web5 protocol implementation
│   │   ├── ml/              # Machine learning components
│   │   ├── dao/             # DAO implementation
│   │   ├── enterprise/      # Enterprise features
│   │   └── extensions/      # Extension system
│   ├── lib/                 # Core library
│   │   └── src/             # Rust library source
│   │       ├── api/         # API implementation with Actix Web
│   │       ├── core/        # Core functionality
│   │       └── automation/  # Automation tools
│   ├── dependencies/        # Core dependencies
│   │   ├── anya-bitcoin/    # Bitcoin-specific implementation
│   │   │   └── src/         # Bitcoin source
│   │   │       ├── rgb/     # RGB protocol implementation
│   │   │       ├── rsk/     # RSK integration
│   │   │       └── stacks/  # Stacks integration
│   │   └── anya-extensions/ # Extension framework
│   ├── tests/               # Test suite
│   └── docs/                # Documentation
├── anya-bitcoin/           # Bitcoin-specific implementation
├── anya/                   # Main framework components
├── web5/                   # Web5 protocol integration
├── mobile/                 # Mobile application source
├── contracts/              # Smart contracts
├── services/               # Supporting services
├── stacks/                 # Stacks blockchain integration
├── ml/                     # Machine learning modules
├── mlops/                  # ML operations system
├── metrics/                # System metrics and monitoring
├── tests/                  # Integration tests
└── docs/                   # Project documentation
```

## Core Components

### 1. Bitcoin Integration Layer

- **Purpose**: Core Bitcoin and Lightning functionality
- **Key Features**:
  - Bitcoin Core & Lightning Network support
  - DLC (Discreet Log Contracts)
  - Taproot/Schnorr signatures
  - Layer 2 solutions
  - Cross-chain capabilities
- **Main Libraries**:
  - rust-bitcoin (v0.32.5): Core data structures and cryptography
  - BDK (v0.30.2): Wallet operations and descriptor-based key management
  - LDK (v0.0.116): Lightning Network protocol implementation
  - RGB Core (v0.10): Asset issuance on Bitcoin
  - Taproot (v0.1.0): Advanced contract functionality
  - rust-dlc (v0.4.0): Discreet Log Contracts implementation

### 2. Web5 Integration Layer

- **Purpose**: Decentralized data management
- **Key Features**:
  - Decentralized Web Nodes (DWN)
  - Decentralized Identifiers (DIDs)
  - Protocol-based data management
  - Identity-centric storage
  - Secure data encryption

### 3. API Layer

- **Purpose**: Web API and real-time communication
- **Key Features**:
  - RESTful API endpoints
  - WebSocket support for real-time updates
  - JWT-based authentication
  - Cross-origin resource sharing
- **Main Libraries**:
  - actix-web (v4.3.1): Web framework
  - jsonwebtoken (v8.3.0): JWT authentication
  - actix-cors (v0.6.4): CORS support
  - actix-web-actors (v4.2.0): WebSocket support

### 4. ML/AI System

- **Purpose**: Advanced analytics and prediction
- **Key Features**:
  - Model optimization
  - Federated learning
  - Transaction pattern recognition
  - Pipeline optimization
  - Prediction models

### 5. Enterprise Features

- **Purpose**: Business operations and integration
- **Key Features**:
  - Risk management
  - Compliance tracking
  - Multi-signature operations
  - Revenue system
  - Business analytics

### 6. Mobile Application

- **Purpose**: Cross-platform mobile interface
- **Key Features**:
  - Wallet management
  - Transaction operations
  - Security features
  - Multi-chain support

## Hexagonal Architecture

The system follows hexagonal architecture principles with these port implementations:

| Port Type       | Bitcoin Protocol      | Web5/DWN              | DAO                   | ML/AI                 |
|-----------------|-----------------------|-----------------------|-----------------------|-----------------------|
| Storage         | Sled DB               | DWN Records           | IPFS+Filecoin         | TensorFlow Storage    |
| Identity        | BIP32 HD Keys         | DID:Web               | DAO NFT Memberships   | Model Signatures      |
| Compute         | Script Interpreter    | Web Workers           | EVM/Solidity          | Federated Learning    |
| Networking      | Bitcoin P2P           | libp2p                | OrbitDB               | Secure Aggregation    |

## Bitcoin Implementation Structure

The Bitcoin functionality is organized following a modular approach:

| Component          | Implementation                       | Description                          |
|--------------------|------------------------------------- |--------------------------------------|
| Consensus          | anya-bitcoin/src/consensus          | Block and transaction validation      |
| Mempool            | anya-bitcoin/src/mempool           | Transaction memory pool management    |
| Networking         | anya-bitcoin/src/net               | P2P network communication             |
| Script             | anya-bitcoin/src/script            | Script verification and execution     |
| Wallet             | BDK integration                      | Key and UTXO management              |
| Lightning          | LDK integration                      | Lightning Network operations         |
| DLC                | anya-bitcoin/src/dlc               | Discreet Log Contracts                |

## Dependency Matrix

| Component          | Dependencies                             | Status      |
|--------------------|----------------------------------------- |-------------|
| Bitcoin Core       | rust-bitcoin v0.32.5, secp256k1 v0.27.0   | Operational |
| Lightning          | LDK v0.0.116                            | Beta        |
| BDK Wallet         | BDK v0.30.2, bitcoincore-rpc v0.17.0      | Alpha       |
| Web5               | web5 v0.1.0, DWN SDK                    | Alpha       |
| ML/AI              | TensorFlow, PyTorch                     | Development |
| Mobile             | Flutter, Dart                           | Alpha       |
| Enterprise         | Aragon OSx, Hyperledger Besu            | Development |

## Integration Points

1. **Bitcoin ↔ Web5**: PSBT management, DID:BTCR
2. **Bitcoin ↔ Lightning**: Channel management, payment routing
3. **Bitcoin ↔ DLC**: Oracle signatures, adaptor signatures
4. **Web5 ↔ ML/AI**: Privacy-preserving data sharing
5. **ML/AI ↔ Enterprise**: Revenue prediction, risk assessment
6. **Enterprise ↔ Bitcoin**: Multi-signature governance
7. **Mobile ↔ All**: Unified interface to all components

## Deployment Architecture

```text
                 ┌───────────────┐
                 │  Mobile App   │
                 └───────┬───────┘
                         │
           ┌─────────────┼─────────────┐
┌──────────┴──────┐ ┌────┴────┐ ┌──────┴──────┐
│ Bitcoin Network │ │ Web5 DWN │ │ AI Services │
└────────┬────────┘ └────┬─────┘ └──────┬──────┘
         │               │              │
         └───────────────┼──────────────┘
                         │
                  ┌──────┴───────┐
                  │ Enterprise   │
                  └──────────────┘
```

## Security Layer

- **Authentication & Identity**: Multi-factor authentication, DID-based identity
- **Blockchain Security**: Taproot/Schnorr signatures, multi-signature support
- **Data Protection**: End-to-end encryption, zero-knowledge proofs
- **ML/AI Security**: Federated learning, differential privacy
- **Enterprise Security**: Role-based access, compliance monitoring

## Monitoring & Metrics

- **System Health**: Real-time monitoring of all components
- **Performance Tracking**: Resource utilization and bottleneck detection
- **Security Monitoring**: Threat detection and compliance validation
- **ML/AI Metrics**: Model accuracy, training efficiency, inference latency
- **Bitcoin Metrics**: Mempool depth, fee estimation, UTXO set analysis

## Migration Strategy (Python to Rust)

| Component          | Current Implementation   | Target Implementation        | Status         |
|--------------------|--------------------------|-----------------------------|----------------|
| Bitcoin Core       | python-bitcoinlib        | rust-bitcoin                | In Progress    |
| Wallet Operations  | python-bitcoinlib        | BDK                         | Planning       |
| RPC Client         | python-bitcoinrpc        | bitcoincore-rpc             | In Progress    |
| Key Management     | hdwallet, base58         | secp256k1, BDK              | Planning       |

## Development Status

| Component          | Status                 | Next Milestone                |
|--------------------|------------------------|-------------------------------|
| Bitcoin Core       | Operational            | Taproot-DLC Integration       |
| Lightning          | Beta                   | Channel Management            |
| DLC                | Alpha                  | Oracle Network Integration    |
| Web5               | Alpha                  | DID Rotation System           |
| ML/AI              | Development            | Federated Learning v2         |
| DAO                | Development            | Governance Upgrade            |
| Enterprise         | Planning               | Compliance System             |
| Mobile             | Alpha                  | RGB Protocol Support          |

*Last updated: 2025-03-01*
