//! Read First Metrics
//!
//! This module provides metrics tracking for the Read First Always principle,
//! helping ensure that all operations read the current state before making any changes.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, Duration};

/// Metrics for tracking Read First Always principle compliance
#[derive(Debug)]
pub struct ReadFirstMetrics {
    /// Name of the component being tracked
    name: String,
    
    /// Number of read operations performed
    read_count: AtomicUsize,
    
    /// Number of write operations performed
    write_count: AtomicUsize,
    
    /// Number of Read First Always principle violations
    violation_count: AtomicUsize,
    
    /// Time of first operation
    start_time: Instant,
    
    /// Cumulative time spent on read operations
    read_time: AtomicUsize,
    
    /// Cumulative time spent on write operations
    write_time: AtomicUsize,
}

impl ReadFirstMetrics {
    /// Create new metrics for a named component
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            read_count: AtomicUsize::new(0),
            write_count: AtomicUsize::new(0),
            violation_count: AtomicUsize::new(0),
            start_time: Instant::now(),
            read_time: AtomicUsize::new(0),
            write_time: AtomicUsize::new(0),
        }
    }
    
    /// Get the component name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get the number of read operations
    pub fn reads(&self) -> usize {
        self.read_count.load(Ordering::SeqCst)
    }
    
    /// Get the number of write operations
    pub fn writes(&self) -> usize {
        self.write_count.load(Ordering::SeqCst)
    }
    
    /// Get the number of violations
    pub fn violations(&self) -> usize {
        self.violation_count.load(Ordering::SeqCst)
    }
    
    /// Calculate read-write ratio
    pub fn read_write_ratio(&self) -> f64 {
        let reads = self.reads();
        let writes = self.writes();
        
        if writes == 0 {
            return if reads == 0 { 0.0 } else { f64::INFINITY };
        }
        
        reads as f64 / writes as f64
    }
    
    /// Get time since start
    pub fn time_since_start(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get average read time in microseconds
    pub fn average_read_time_micros(&self) -> f64 {
        let reads = self.reads();
        if reads == 0 {
            return 0.0;
        }
        
        self.read_time.load(Ordering::SeqCst) as f64 / reads as f64
    }
    
    /// Get average write time in microseconds
    pub fn average_write_time_micros(&self) -> f64 {
        let writes = self.writes();
        if writes == 0 {
            return 0.0;
        }
        
        self.write_time.load(Ordering::SeqCst) as f64 / writes as f64
    }
    
    /// Increment the read counter
    pub fn increment_reads(&self) {
        self.read_count.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Increment the write counter
    pub fn increment_writes(&self) {
        self.write_count.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Increment the violation counter
    pub fn increment_violations(&self) {
        self.violation_count.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Record time for a read operation in microseconds
    pub fn record_read_time(&self, micros: u64) {
        self.read_time.fetch_add(micros as usize, Ordering::SeqCst);
    }
    
    /// Record time for a write operation in microseconds
    pub fn record_write_time(&self, micros: u64) {
        self.write_time.fetch_add(micros as usize, Ordering::SeqCst);
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        self.read_count.store(0, Ordering::SeqCst);
        self.write_count.store(0, Ordering::SeqCst);
        self.violation_count.store(0, Ordering::SeqCst);
        self.read_time.store(0, Ordering::SeqCst);
        self.write_time.store(0, Ordering::SeqCst);
    }
    
    /// Create report summary as string
    pub fn summary(&self) -> String {
        format!(
            "ReadFirstMetrics[{}] - Reads: {}, Writes: {}, Violations: {}, Ratio: {:.2}, Avg Read: {:.2}µs, Avg Write: {:.2}µs, Duration: {:?}",
            self.name,
            self.reads(),
            self.writes(),
            self.violations(),
            self.read_write_ratio(),
            self.average_read_time_micros(),
            self.average_write_time_micros(),
            self.time_since_start()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    
    #[test]
    fn test_read_first_metrics() {
        let metrics = ReadFirstMetrics::new("test");
        
        // Test initial state
        assert_eq!(metrics.reads(), 0);
        assert_eq!(metrics.writes(), 0);
        assert_eq!(metrics.violations(), 0);
        assert_eq!(metrics.read_write_ratio(), 0.0);
        
        // Test incrementing reads
        metrics.increment_reads();
        metrics.increment_reads();
        assert_eq!(metrics.reads(), 2);
        assert_eq!(metrics.writes(), 0);
        assert_eq!(metrics.read_write_ratio(), f64::INFINITY);
        
        // Test incrementing writes
        metrics.increment_writes();
        assert_eq!(metrics.reads(), 2);
        assert_eq!(metrics.writes(), 1);
        assert_eq!(metrics.read_write_ratio(), 2.0);
        
        // Test incrementing violations
        metrics.increment_violations();
        assert_eq!(metrics.violations(), 1);
        
        // Test recording times
        metrics.record_read_time(100);
        metrics.record_read_time(200);
        metrics.record_write_time(300);
        
        assert_eq!(metrics.average_read_time_micros(), 150.0);
        assert_eq!(metrics.average_write_time_micros(), 300.0);
        
        // Test reset
        metrics.reset();
        assert_eq!(metrics.reads(), 0);
        assert_eq!(metrics.writes(), 0);
        assert_eq!(metrics.violations(), 0);
        assert_eq!(metrics.average_read_time_micros(), 0.0);
        assert_eq!(metrics.average_write_time_micros(), 0.0);
    }
    
    #[test]
    fn test_read_first_metrics_threading() {
        let metrics = std::sync::Arc::new(ReadFirstMetrics::new("threading_test"));
        let threads_count = 10;
        let reads_per_thread = 5;
        let writes_per_thread = 2;
        
        let mut handles = vec![];
        
        for _ in 0..threads_count {
            let metrics_clone = metrics.clone();
            
            let handle = thread::spawn(move || {
                for _ in 0..reads_per_thread {
                    metrics_clone.increment_reads();
                    thread::sleep(std::time::Duration::from_micros(1));
                }
                
                for _ in 0..writes_per_thread {
                    metrics_clone.increment_writes();
                    thread::sleep(std::time::Duration::from_micros(1));
                }
                
                // Record some random times
                metrics_clone.record_read_time(100);
                metrics_clone.record_write_time(200);
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify results
        assert_eq!(metrics.reads(), threads_count * reads_per_thread);
        assert_eq!(metrics.writes(), threads_count * writes_per_thread);
        
        // Each thread recorded 100 micros per read (1 time per thread)
        let expected_avg_read = (threads_count as f64 * 100.0) / (threads_count * reads_per_thread) as f64;
        
        // Allow some margin of error for floating point comparison
        assert!((metrics.average_read_time_micros() - expected_avg_read).abs() < 0.01);
        
        // Test summary string
        let summary = metrics.summary();
        assert!(summary.contains("ReadFirstMetrics[threading_test]"));
        assert!(summary.contains(&format!("Reads: {}", threads_count * reads_per_thread)));
        assert!(summary.contains(&format!("Writes: {}", threads_count * writes_per_thread)));
    }
}
