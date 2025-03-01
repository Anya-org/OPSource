# OPSource Project Index

Last Updated: March 2, 2025

## Core Platform Components
- [Anya Core](./anya/INDEX.md) - Core Platform Components
  - [Dependencies](./anya/dependencies/INDEX.md) - Core Dependencies & Libraries
  - [Enterprise](./anya/enterprise/INDEX.md) - Enterprise Features & Solutions
  - [Mobile](./anya/mobile/INDEX.md) - Mobile Application Source
- [Dash33](./dash33/INDEX.md) - Analytics Dashboard & ML Integration

## Bitcoin Implementation
- [Bitcoin Core](./anya-core/dependencies/anya-bitcoin/README.md) - Core Bitcoin Implementation
  - [Consensus](./anya-core/dependencies/anya-bitcoin/src/consensus/README.md) - Consensus Rules
  - [Mempool](./anya-core/dependencies/anya-bitcoin/src/mempool/README.md) - Transaction Mempool
  - [Network](./anya-core/dependencies/anya-bitcoin/src/net/README.md) - P2P Networking
  - [Script](./anya-core/dependencies/anya-bitcoin/src/script/README.md) - Script Execution
  - [DLC](./anya-core/dependencies/anya-bitcoin/src/dlc/README.md) - Discreet Log Contracts
  - [RGB](./anya-core/dependencies/anya-bitcoin/src/rgb/README.md) - RGB Protocol Implementation
  - [RSK](./anya-core/dependencies/anya-bitcoin/src/rsk/README.md) - RSK Smart Contract Integration
  - [Stacks](./anya-core/dependencies/anya-bitcoin/src/stacks/README.md) - Stacks Blockchain Integration
- [Rust Bitcoin Libraries](./rust-bitcoin-libraries.md) - Rust Bitcoin Library Documentation
  - [BDK Example](./bdk-wallet-example.rs) - Bitcoin Development Kit Example
  - [Rust-Bitcoin Example](./rust-bitcoin-example.rs) - Basic Rust-Bitcoin Example
  - [Cargo Configuration](./Cargo.toml.example) - Example Cargo Configuration

## API Layer
- [API Documentation](./anya-core/lib/src/api/README.md) - API Layer Documentation
  - [Authentication](./anya-core/lib/src/api/auth.rs) - JWT Authentication Implementation
  - [WebSocket](./anya-core/lib/src/api/websocket.rs) - WebSocket Implementation
  - [Routes](./anya-core/lib/src/api/routes.rs) - API Routes Implementation
  - [Server Configuration](./anya-core/lib/src/api/mod.rs) - Actix Web Server Configuration

## Advanced Analytics
- [Real-time Processing](./dash33/analytics/realtime/README.md)
- [Cross-chain Analysis](./dash33/analytics/cross_chain/README.md)
- [Pattern Recognition](./dash33/analytics/patterns/README.md)
- [Machine Learning Models](./dash33/ml/README.md)

## Enterprise Solutions
- [DAO Governance](./enterprise/governance/README.md)
- [Multi-chain Support](./enterprise/multichain/README.md)
- [Security Features](./enterprise/security/README.md)
- [Reporting System](./enterprise/reporting/README.md)

## Mobile Applications
- [iOS Application](./mobile/ios/README.md)
- [Android Application](./mobile/android/README.md)
- [Wallet Integration](./mobile/wallet/README.md)
- [Push Notifications](./mobile/notifications/README.md)

## Development Resources
- [Getting Started Guide](./docs/getting-started.md)
- [API Documentation](./docs/api.md)
- [Contributing Guide](./CONTRIBUTING.md)
- [Development Reports](./reports/development_summary.md)

## Project Management
- [Development Roadmap](./ROADMAP.md)
- [Changelog](./CHANGELOG.md)
- [Issue Tracker](https://github.com/botshelomokoka/OPSource/issues)
- [Project Status](./reports/status/README.md)

## Configuration & Setup
- [Environment Setup](./config/example.env)
- [Development Scripts](./scripts/README.md)
- [GitHub Workflows](./.github/workflows/README.md)
- [Development Tracker](./scripts/track_development.py)

## Community & Support
- [Discord Community](https://discord.gg/opsource)
- [Twitter Updates](https://twitter.com/OPSource)
- [Security Policy](./SECURITY.md)
- [Code of Conduct](./CODE_OF_CONDUCT.md)

## Migration Guide
- [Python to Rust Migration](./docs/migration/python_to_rust.md) - Guide for migrating from Python-bitcoinlib to Rust
- [Integration Patterns](./docs/integration/bitcoin_integration.md) - Patterns for Bitcoin integration
- [Testing Guide](./docs/testing/bitcoin_testing.md) - Bitcoin-specific testing guidelines
