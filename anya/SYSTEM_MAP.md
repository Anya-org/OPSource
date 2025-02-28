# Anya System Architecture Map

## Repository Structure

```
anya (anya-core)
├── anya-bitcoin/               # Bitcoin integration
├── anya-enterprise/           # Enterprise core features
├── anya-extensions/          # Extension system
├── dash33/                  # Web dashboard
├── dependencies/           # Shared dependencies
├── enterprise/           # Business implementation
└── mobile/             # Cross-platform mobile app
```

## Core Components

### 1. Bitcoin Integration ([anya-bitcoin](./anya-bitcoin/))

- **Purpose**: Core Bitcoin functionality
- **Documentation**: [Bitcoin Integration Guide](./anya-bitcoin/docs/README.md)
- **Key Files**:
  - [Cargo.toml](./anya-bitcoin/Cargo.toml)
  - [Source](./anya-bitcoin/src/)

### 2. Enterprise Core ([anya-enterprise](./anya-enterprise/))

- **Documentation**: [Enterprise README](./anya-enterprise/README.md)
- **Changelog**: [CHANGELOG.md](./anya-enterprise/CHANGELOG.md)
- **Architecture**: [Source Directory](./anya-enterprise/src/)

### 3. Dashboard (dash33)

- **Web Interface**: [main.py](./dash33/web/main.py)
- **Components**:
  - [Wallet Manager](./dash33/wallet/wallet_manager.py)
  - [Security](./dash33/core/security.py)
  - [AI Analysis](./dash33/ai/analyzer.py)

### 4. Mobile Application

- **Platforms**: Android, iOS, Desktop
- **Documentation**:
  - [Development Guide](./mobile/DEVELOPMENT.md)
  - [Technical TODO](./mobile/TECHNICAL_TODO.md)
  - [Roadmap](./mobile/ROADMAP.md)
- **Source**: [lib directory](./mobile/lib/)

## Documentation Index

### Architecture Documents

- [Agent Architecture](./AGENT_ARCHITECTURE.md)
- [DAO Structure](./DAO.md)
- [Governance](./GOVERNANCE.md)

### Development Guides

- [Contributing Guide](./CONTRIBUTING.md)
- [Security Policy](./SECURITY.md)
- [Testing Guide](./TESTING.md)

### Planning & Roadmap

- [New Features](./NEW_FEATURES.md)
- [Roadmap](./ROADMAP.md)
- [Changelog](./CHANGELOG.md)

## Configuration Files

### Core Configuration

- [Cargo.toml](./Cargo.toml) - Rust dependencies
- [.env.template](./.env.template) - Environment template
- [docker-compose.yml](./docker-compose.yml) - Container orchestration

### Build & CI

- [build.rs](./build.rs) - Rust build script
- [rust_combined.yml](./rust_combined.yml) - CI pipeline

## Scripts

- [commit_push.ps1](./commit_push.ps1) - Git automation
- [install_dependencies.sh](./install_dependencies.sh) - Setup script
- [reorganize-code.ps1](./reorganize-code.ps1) - Code organization

## Symbolic Links

The following components are symlinked:

- `/anya/dash33` → `[Repository Root]/dash33`
- `/anya/enterprise` → `[Repository Root]/enterprise`
- `/anya/mobile` → `[Repository Root]/mobile`

## System Requirements

- Rust toolchain
- Python 3.8+
- Flutter SDK
- Docker & Docker Compose

## Quick Links

- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [License](./LICENSE.md)
- [Security Policy](./SECURITY.md)

## Updated Component Matrix

| Component              | Tech Stack                  | Security Level | Dependencies          |
|------------------------|-----------------------------|----------------|-----------------------|
| DLC Engine             | rust-dlc, schnorr_fun       | HSM-Protected  | rust-bitcoin v0.30+   |
| Taproot Services       | bdk-taproot, taproot-assets | Tier 1         | Bitcoin Core 25+      |
| RSK Bridge             | powpeg-rs, web3-rs          | Tier 2         | RSKj 4.0+             |
| AI Oracles             | TF-Encrypted, FHE           | Tier 0         | SGX Enclaves          |
| Mobile SDK             | React Native LDK            | Tier 1         | Android HSM           |

## Critical Path Dependencies

1. Bitcoin Core 25+ Taproot improvements
2. LDK v0.8 DLC support (Q4 2024)
3. RSK Powpeg v3 security upgrade

## Stacks Operations (Enhanced)

### Core Components

1. **Clarity Contracts**
   - [Asset Issuance](./contracts/stacks/assets.clar)
   - [BTC Anchor Verification](./contracts/stacks/btc_anchor.clar)
   - [DLC Settlement](./contracts/stacks/dlc.clar)

2. **Bridge Services**
   - [BTC→STX Burn/Mint](./services/stacks/bridge.rs)
   - [State Proof Relay](./services/stacks/proof_relay.md)
   - [Microblock Repeater](./services/stacks/microblocks.rs)

3. **Monitoring**
   - [Stacking Pool Health](./metrics/stacks/pools.md)
   - [Clarity VM Metrics](./metrics/stacks/vm_performance.md)
   - [Bridge Finality](./metrics/stacks/bridge_finality.md)

## Hexagonal Architecture Mapping

### Core Domains

- **Bitcoin Protocol**
  - Adapters: rust-bitcoin, LDK, BDK
  - Status: Production

- **Decentralized Web**
  - Adapters: Web5.js, DWN SDK
  - Status: Beta

- **Enterprise Services**
  - Adapters: Aragon OSx, Hyperledger Besu
  - Status: Staging

- **AI/ML Systems**
  - Adapters: TF Encrypted, FATE
  - Status: Active Development

### Port Implementations

| Port Type       | Bitcoin Protocol      | Web5/DWN              | DAO                   |
|-----------------|-----------------------|-----------------------|-----------------------|
| Storage         | Sled DB               | DWN Records           | IPFS+Filecoin         |
| Identity        | BIP32 HD Keys         | DID:Web               | DAO NFT Memberships   |
| Compute         | Script Interpreter    | Web Workers           | EVM/Solidity          |
| Networking      | Bitcoin P2P           | libp2p                | OrbitDB               |

## Hexagonal Ports

### Core Domains

- **Bitcoin Protocol**
  - Adapters: rust-bitcoin, LDK, BDK
  - Status: Production

- **Decentralized Web**
  - Adapters: Web5.js, DWN SDK
  - Status: Beta

- **Enterprise Services**
  - Adapters: Aragon OSx, Hyperledger Besu
  - Status: Staging

- **AI/ML Systems**
  - Adapters: TF Encrypted, FATE
  - Status: Active Development

### Port Implementations

| Port Type       | Bitcoin Protocol      | Web5/DWN              | DAO                   |
|-----------------|-----------------------|-----------------------|-----------------------|
| Storage         | Sled DB               | DWN Records           | IPFS+Filecoin         |
| Identity        | BIP32 HD Keys         | DID:Web               | DAO NFT Memberships   |
| Compute         | Script Interpreter    | Web Workers           | EVM/Solidity          |
| Networking      | Bitcoin P2P           | libp2p                | OrbitDB               |

---
*This map is automatically updated through CI/CD pipelines. Last updated: 2025-02-28*

graph TD
    UA[User Action] --> WA{Web5 Auth}
    WA -->|DID Valid| DL[Decentralized Ledger]
    WA -->|Invalid| RX[Reject]

    DL --> CT{Contract Type}
    CT -->|DLC| OC[Oracle Consensus]
    CT -->|RGB| SC[State Commit]
    CT -->|sBTC| BV[Bridge Verify]
    
    OC -->|Multi-Sig| ML[ML Predict]
    SC -->|ZK Proof| DG[DAO Vote]
    BV -->|Confirms≥6| RC[Reserve Check]
    
    ML -->|Threshold| SB[Sign Broadcast]
    DG -->|Time-Lock| GE[Govern Execute]
    RC -->|Collateral≥90%| MB[Mint/Burn]
    
    SB --> TX[On-Chain]
    GE --> UP[Protocol Update]
    MB --> AS[Asset State]
    
    classDef security fill:#f9f,stroke:#333;
    class OC,RC,GE security;

# Updated System Architecture

## Core Modules (v0.4.1)

### Bitcoin Protocol Layer

- **Enhanced DLC Engine**
  - File: `anya-bitcoin/src/dlc/oracle.rs`
  - Features:
    - Multi-Oracle Schnorr Verification
    - 6h Dispute Timeout
    - HSM-Secured Signing

- **DAO Governance V2**
  - File: `contracts/dao/Governance.sol`
  - Updates:
    - 2-Day Execution Delay
    - 20% Veto Threshold
    - Quadratic Voting Support

### Web5 Integration

- **PSBT Schema System**
  - File: `web5/psbt.schema.json`
  - Validation:
    - Taproot Version Enforcement
    - XPUB Format Checking
    - Merkle Proof Structure

- **DID-Bitcoin Bridge**
  - File: `web5/bitcoin.ts`
  - Methods:
    - DID:BTCR Resolution
    - PSBT ↔ DWN Mapping
    - Key Rotation Scheduler

### Security Subsystem

- **sBTC Safeguards**
  - File: `services/stacks/bridge.rs`
  - Rules:
    - 6-Confirmation Minimum
    - 90% Collateral Floor
    - Reserve Health Monitor

- **ML Integrity**
  - File: `ml/federated.py`
  - Protections:
    - Model Weight Signing
    - TEE Verification
    - Federated Blacklist

## Critical Data Flows

1. `User Request → Web5 DID Auth → Contract Selection → Oracle/DAO Path`
2. `DLC Outcome → Multi-Oracle Consensus → ML Adjustment → Settlement`
3. `sBTC Mint → Reserve Check → Collateral Lock → Asset Issue`

## Attack Surface Mitigations

| Threat Vector          | Mitigation Layer           | Detection Method               |
|------------------------|----------------------------|--------------------------------|
| Oracle Collusion       | DLC Multi-Sig Consensus    | Threshold Signature Analysis   |
| Governance Takeover     | Quadratic Voting + Veto    | Vote Distribution Monitoring   |
| PSBT Injection         | Schema Validation          | DWN Structure Enforcement      |
| Model Poisoning         | Weight Signing + TEE       | Gradient Anomaly Detection     |
| Bridge Reserve Drain    | Realtime Collateral Check  | Liquidity Health Alerts        |

