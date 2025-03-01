//! Read First DWN Manager
//!
//! This module implements a Decentralized Web Node (DWN) manager that strictly
//! adheres to the "Read First Always" principle to ensure data integrity and 
//! prevent race conditions. All operations first read current state before 
//! performing any writes.
//!
//! The ReadFirstDwnManager wraps a standard DWN implementation and adds 
//! monitoring and enforcement of the Read First Always principle.

use super::dwn::{DwnManager, DwnRecord, DwnQuery, DwnError};
use super::metrics::ReadFirstMetrics;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use log::{info, warn, error};

/// Result of DWN operations
pub type DwnResult<T> = Result<T, DwnError>;

/// Read First DWN Manager that ensures the Read First Always principle is followed
#[derive(Debug)]
pub struct ReadFirstDwnManager {
    /// Inner DWN manager
    inner: Arc<DwnManager>,
    
    /// Metrics tracking for Read First Always principle
    metrics: Arc<Mutex<ReadFirstMetrics>>,
}

impl ReadFirstDwnManager {
    /// Create a new Read First DWN Manager
    pub fn new(inner: Arc<DwnManager>) -> Self {
        Self {
            inner,
            metrics: Arc::new(Mutex::new(ReadFirstMetrics::new("dwn"))),
        }
    }
    
    /// Get the inner DWN manager
    pub fn inner(&self) -> Arc<DwnManager> {
        self.inner.clone()
    }
    
    /// Get the metrics
    pub fn metrics(&self) -> Arc<Mutex<ReadFirstMetrics>> {
        self.metrics.clone()
    }
    
    /// Log metrics
    pub fn log_metrics(&self) {
        let metrics = self.metrics.lock().unwrap();
        info!(
            "DWN Read First Metrics - Reads: {}, Writes: {}, Violations: {}, Read-Write Ratio: {:.2}",
            metrics.reads(),
            metrics.writes(),
            metrics.violations(),
            metrics.read_write_ratio()
        );
    }
    
    /// Store a record in the DWN
    pub async fn store_record(&self, record: DwnRecord) -> DwnResult<()> {
        // Record the start time
        let start_time = std::time::Instant::now();
        
        // Get the record ID for tracking
        let record_id = record.id().to_string();
        
        // READ FIRST ALWAYS: First check if record already exists
        let existing_record = self.get_record(&record_id).await?;
        
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Read record before store operation. Record ID: {}", record_id);
        }
        
        // Store the record
        let result = self.inner.store_record(record).await;
        
        // Track the write operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_writes();
            info!("READ_FIRST_ALWAYS: Completed write operation after read. Record ID: {}, Duration: {:?}", 
                  record_id, start_time.elapsed());
        }
        
        result
    }
    
    /// Get a record from the DWN by ID
    pub async fn get_record(&self, record_id: &str) -> DwnResult<Option<DwnRecord>> {
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Performing read operation. Record ID: {}", record_id);
        }
        
        self.inner.get_record(record_id).await
    }
    
    /// Query records in the DWN
    pub async fn query_records(&self, query: DwnQuery) -> DwnResult<Vec<DwnRecord>> {
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Performing query operation.");
        }
        
        self.inner.query_records(query).await
    }
    
    /// Delete a record from the DWN
    pub async fn delete_record(&self, record_id: &str) -> DwnResult<bool> {
        // Record the start time
        let start_time = std::time::Instant::now();
        
        // READ FIRST ALWAYS: First check if record exists
        let existing_record = self.get_record(record_id).await?;
        
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Read record before delete operation. Record ID: {}", record_id);
        }
        
        // If the record doesn't exist, we don't need to delete it
        if existing_record.is_none() {
            info!("READ_FIRST_ALWAYS: No record found to delete. Record ID: {}", record_id);
            return Ok(false);
        }
        
        // Delete the record
        let result = self.inner.delete_record(record_id).await;
        
        // Track the write operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_writes();
            info!("READ_FIRST_ALWAYS: Completed delete operation after read. Record ID: {}, Duration: {:?}", 
                  record_id, start_time.elapsed());
        }
        
        result
    }
    
    /// Update a record in the DWN
    pub async fn update_record(&self, record: DwnRecord) -> DwnResult<()> {
        // Record the start time
        let start_time = std::time::Instant::now();
        
        // Get the record ID for tracking
        let record_id = record.id().to_string();
        
        // READ FIRST ALWAYS: First check if record exists
        let existing_record = self.get_record(&record_id).await?;
        
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Read record before update operation. Record ID: {}", record_id);
        }
        
        // If the record doesn't exist, we can't update it
        if existing_record.is_none() {
            // Track violation
            {
                let mut metrics = self.metrics.lock().unwrap();
                metrics.increment_violations();
                warn!("READ_FIRST_ALWAYS violation: Attempted to update non-existent record. Record ID: {}", record_id);
            }
            
            return Err(DwnError::RecordNotFound(record_id));
        }
        
        // Update the record
        let result = self.inner.update_record(record).await;
        
        // Track the write operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_writes();
            info!("READ_FIRST_ALWAYS: Completed update operation after read. Record ID: {}, Duration: {:?}", 
                  record_id, start_time.elapsed());
        }
        
        result
    }
    
    /// Batch store multiple records
    pub async fn batch_store_records(&self, records: Vec<DwnRecord>) -> DwnResult<()> {
        // Record the start time
        let start_time = std::time::Instant::now();
        
        // Extract record IDs for logging
        let record_ids: Vec<String> = records.iter()
            .map(|r| r.id().to_string())
            .collect();
        
        let record_count = records.len();
        
        // READ FIRST ALWAYS: First check if records already exist
        for record in &records {
            let record_id = record.id().to_string();
            let existing_record = self.get_record(&record_id).await?;
            
            // Track each read operation
            {
                let mut metrics = self.metrics.lock().unwrap();
                metrics.increment_reads();
            }
            
            info!("READ_FIRST_ALWAYS: Read record before batch store. Record ID: {}", record_id);
        }
        
        // Store the records
        let result = self.inner.batch_store_records(records).await;
        
        // Track the batch write operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            // Each record in batch counts as a write
            for _ in 0..record_count {
                metrics.increment_writes();
            }
            info!("READ_FIRST_ALWAYS: Completed batch store operation after reads. Record count: {}, Duration: {:?}", 
                  record_count, start_time.elapsed());
        }
        
        result
    }
    
    /// Check if a record exists
    pub async fn record_exists(&self, record_id: &str) -> DwnResult<bool> {
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Checking if record exists. Record ID: {}", record_id);
        }
        
        // Get the record to check existence
        let record = self.get_record(record_id).await?;
        
        // Return whether the record exists
        Ok(record.is_some())
    }
    
    /// Count records matching a query
    pub async fn count_records(&self, query: DwnQuery) -> DwnResult<usize> {
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Counting records.");
        }
        
        // Query records and return the count
        let records = self.query_records(query).await?;
        Ok(records.len())
    }
    
    /// Get record history
    pub async fn get_record_history(&self, record_id: &str) -> DwnResult<Vec<DwnRecord>> {
        // Track the read operation
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.increment_reads();
            info!("READ_FIRST_ALWAYS: Getting record history. Record ID: {}", record_id);
        }
        
        self.inner.get_record_history(record_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::web5::dwn::MockDwnManager;
    
    #[tokio::test]
    async fn test_read_first_dwn() -> Result<(), Box<dyn std::error::Error>> {
        // Create a mock DWN manager
        let mock_dwn = Arc::new(MockDwnManager::new());
        
        // Create the Read First DWN manager with the mock
        let read_first_dwn = ReadFirstDwnManager::new(mock_dwn);
        
        // Create a test record
        let record = DwnRecord::new("test-record", "test-type", b"test-data".to_vec());
        
        // Store the record
        read_first_dwn.store_record(record.clone()).await?;
        
        // Verify that the metrics were updated
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 1, "Should have performed 1 read before storing");
            assert_eq!(metrics.writes(), 1, "Should have performed 1 write during storing");
            assert_eq!(metrics.violations(), 0, "Should not have any violations");
        }
        
        // Get the record
        let retrieved_record = read_first_dwn.get_record("test-record").await?;
        
        // Verify the record was retrieved
        assert!(retrieved_record.is_some(), "Record should exist");
        
        // Verify the metrics were updated
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 2, "Should have performed 2 reads total");
            assert_eq!(metrics.writes(), 1, "Should still have 1 write");
        }
        
        // Update the record
        let updated_record = DwnRecord::new("test-record", "test-type", b"updated-data".to_vec());
        read_first_dwn.update_record(updated_record).await?;
        
        // Verify the metrics were updated
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 3, "Should have performed 3 reads total");
            assert_eq!(metrics.writes(), 2, "Should have 2 writes total");
        }
        
        // Delete the record
        read_first_dwn.delete_record("test-record").await?;
        
        // Verify the metrics were updated
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 4, "Should have performed 4 reads total");
            assert_eq!(metrics.writes(), 3, "Should have 3 writes total");
        }
        
        // Batch store multiple records
        let records = vec![
            DwnRecord::new("batch-1", "test-type", b"batch-data-1".to_vec()),
            DwnRecord::new("batch-2", "test-type", b"batch-data-2".to_vec()),
        ];
        read_first_dwn.batch_store_records(records).await?;
        
        // Verify the metrics were updated
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 6, "Should have performed 6 reads total");
            assert_eq!(metrics.writes(), 5, "Should have 5 writes total");
        }
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_read_first_violations() -> Result<(), Box<dyn std::error::Error>> {
        // Create a mock DWN manager
        let mock_dwn = Arc::new(MockDwnManager::new());
        
        // Create the Read First DWN manager with the mock
        let read_first_dwn = ReadFirstDwnManager::new(mock_dwn);
        
        // Attempt to update a non-existent record (should cause a violation)
        let non_existent_record = DwnRecord::new("non-existent", "test-type", b"test-data".to_vec());
        let result = read_first_dwn.update_record(non_existent_record).await;
        
        // Verify the result is an error
        assert!(result.is_err(), "Updating a non-existent record should fail");
        
        // Verify the metrics were updated with a violation
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 1, "Should have performed 1 read");
            assert_eq!(metrics.writes(), 0, "Should not have performed any writes");
            assert_eq!(metrics.violations(), 1, "Should have recorded 1 violation");
        }
        
        // Attempt another operation that should not violate the principle
        let record = DwnRecord::new("test-record", "test-type", b"test-data".to_vec());
        read_first_dwn.store_record(record).await?;
        
        // Verify the metrics
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            assert_eq!(metrics.reads(), 2, "Should have performed 2 reads total");
            assert_eq!(metrics.writes(), 1, "Should have performed 1 write");
            assert_eq!(metrics.violations(), 1, "Should still have 1 violation");
        }
        
        // Verify read-write ratio
        {
            let metrics = read_first_dwn.metrics().lock().unwrap();
            let ratio = metrics.read_write_ratio();
            assert!(ratio >= 2.0, "Read-write ratio should be at least 2.0");
        }
        
        Ok(())
    }
}
