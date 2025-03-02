# Testing the Read First Always Implementation

This guide explains how to test the Read First Always principle implementation in both Dart and Rust components of the Anya Core project.

## Prerequisites

1. Make sure you have the following development tools installed:
   - Dart SDK (2.19 or higher)
   - Flutter (3.7 or higher)
   - Rust (1.70 or higher)
   - Cargo package manager

2. Clone the repository and check out the feature branch:
   ```bash
   git clone https://github.com/Anya-org/OPSource.git
   cd OPSource
   git checkout feature/read-first-always-main
   ```

## Testing the Dart Implementation

### Running the Unit Tests

1. Navigate to the anya-core directory:
   ```bash
   cd anya-core
   ```

2. Run the Dart tests specifically for the Read First implementation:
   ```bash
   flutter test test/web5/read_first_test.dart
   ```

3. To run all tests including the Read First implementation:
   ```bash
   flutter test
   ```

### Manual Testing

1. You can manually test the Read First implementation by integrating it into your application:

   ```dart
   import 'package:anya_core/src/core/web5/web5_service.dart';
   
   // Initialize the Web5 service (automatically uses Read First)
   final web5Service = await Web5Service.connect();
   
   // Create a record (will automatically read similar records first)
   await web5Service.createRecord(
     collection: 'notes',
     data: {'content': 'This is a test note'},
     schema: 'https://schema.org/TextDigitalDocument',
   );
   
   // Get metrics to verify Read First compliance
   final metrics = web5Service.getReadFirstMetrics();
   print('Read count: ${metrics["read_count"]}');
   print('Write count: ${metrics["write_count"]}');
   print('Compliance rate: ${metrics["compliance_rate"]}%');
   ```

## Testing the Rust Implementation

### Running the Unit Tests

1. Navigate to the anya-core directory:
   ```bash
   cd anya-core
   ```

2. Run the Rust tests specifically for the Web5 agent:
   ```bash
   cargo test --package anya-core --lib src/ml/agents/web5_agent
   ```

3. To run all tests including the Web5 agent:
   ```bash
   cargo test
   ```

### Manual Testing

1. You can manually test the Rust implementation by integrating it into your application:

   ```rust
   use anya_core::ml::agents::web5_agent::{ReadFirstDwnManager, CreateRecordOptions};
   use std::sync::Arc;
   
   // Initialize the Web5 client and wrap it with ReadFirstDwnManager
   let web5_client = get_web5_client();
   let manager = ReadFirstDwnManager::new(Arc::new(web5_client));
   
   // Create a record (will automatically query similar records first)
   let record = manager.create_record(&CreateRecordOptions {
       data: serde_json::to_string(&data)?,
       schema: "https://schema.org/TextDigitalDocument".to_string(),
       data_format: "application/json".to_string(),
   })?;
   
   // Get metrics to verify Read First compliance
   let metrics = manager.get_metrics();
   println!("Read count: {}", metrics.read_count);
   println!("Write count: {}", metrics.write_count);
   println!("Compliance rate: {}%", metrics.compliance_rate());
   ```

## Testing Bitcoin Anchoring with Read First

The Read First principle is particularly important when working with Bitcoin-anchored data. To test this integration:

1. Create a Bitcoin-anchored credential with Read First enforcement:

   ```dart
   // Dart implementation
   final credential = await web5Service.createVerifiableCredential(
     subject: 'did:example:123',
     claims: {'name': 'Test User'},
     bitcoinAnchoring: true, // Enable Bitcoin anchoring
   );
   
   // Check Read First metrics to verify compliance
   final metrics = web5Service.getReadFirstMetrics();
   ```

2. For the Rust implementation:

   ```rust
   // Rust implementation
   let credential = web5_manager.create_verifiable_credential(
       &subject_did,
       &claims,
       Some(BitcoinAnchoringOptions {
           enabled: true,
           confirmation_target: 1,
       }),
   )?;
   
   // Check Read First metrics to verify compliance
   let metrics = web5_manager.get_metrics();
   ```

## Verifying Metrics

To verify the Read First principle is being enforced:

1. Check that the read count is equal to or greater than the write count
2. Verify that the compliance rate is 100%
3. Confirm that the violation count is 0

## Simulating Violations

For testing purposes, you can simulate violations by directly using the underlying Web5 client:

```dart
// This will NOT follow the Read First principle
// Only use for testing, never in production code
final web5Client = web5Service.getUnderlyingWeb5Client();
await web5Client.dwn.records.create(options);

// Now check metrics to see the violation
final metrics = web5Service.getReadFirstMetrics();
print('Violation count: ${metrics["violation_count"]}');
```

## Automated Testing in CI/CD

The Read First principle tests are automatically run in CI/CD pipelines. To check the latest test results:

1. Visit the GitHub Actions page for the repository
2. Look for the "Run Tests" workflow
3. Check the "Test Read First Implementation" job

## Reporting Issues

If you encounter any issues with the Read First implementation, please report them on the GitHub issue tracker with the following information:

1. Steps to reproduce the issue
2. Expected behavior
3. Actual behavior
4. Metrics values (read count, write count, violation count)
5. Any error messages or logs
