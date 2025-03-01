use std::process::Command;
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct SystemDetector {
    system_info: Option<SystemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_type: String,
    pub os_version: String,
    pub cpu_cores: u32,
    pub memory_total_mb: u64,
    pub disk_space_free_gb: u64,
    pub has_nvidia_gpu: bool,
    pub has_amd_gpu: bool,
    pub gpu_info: Option<GpuInfo>,
    pub network_info: NetworkInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub gpu_type: String,
    pub model: String,
    pub memory_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub internet_available: bool,
    pub download_speed_mbps: Option<f64>,
    pub upload_speed_mbps: Option<f64>,
}

impl SystemDetector {
    pub fn new() -> Self {
        Self {
            system_info: None,
        }
    }
    
    /// Check if system meets minimum requirements
    pub fn check_system_requirements(&self) -> Result<()> {
        println!("Checking system requirements...");
        
        // Detect system information
        let info = self.detect_system_info()?;
        
        // Check memory requirements (4GB minimum)
        if info.memory_total_mb < 4096 {
            println!("⚠️ Warning: System has less than 4GB of RAM ({}MB). Performance may be limited.",
                info.memory_total_mb);
        }
        
        // Check disk space requirements (10GB minimum)
        if info.disk_space_free_gb < 10 {
            println!("⚠️ Warning: System has less than 10GB of free disk space ({}GB). Installation may fail.",
                info.disk_space_free_gb);
        }
        
        // Check CPU cores (at least 2 cores recommended)
        if info.cpu_cores < 2 {
            println!("⚠️ Warning: System has fewer than 2 CPU cores ({}). Performance may be limited.",
                info.cpu_cores);
        }
        
        // Check network connectivity
        if !info.network_info.internet_available {
            println!("⚠️ Warning: No internet connectivity detected. Installation may fail.");
        }
        
        println!("✓ System requirements check completed");
        Ok(())
    }
    
    /// Detect system information
    pub fn detect_system_info(&self) -> Result<SystemInfo> {
        // Return cached info if available
        if let Some(info) = &self.system_info {
            return Ok(info.clone());
        }
        
        // Get OS information
        let os_type = std::env::consts::OS.to_string();
        let os_version = self.detect_os_version()?;
        
        // Get CPU cores
        let cpu_cores = sys_info::cpu_num().context("Failed to get CPU cores")? as u32;
        
        // Get memory information
        let mem_info = sys_info::mem_info().context("Failed to get memory info")?;
        let memory_total_mb = mem_info.total / 1024; // Convert KB to MB
        
        // Get disk space
        let disk_info = sys_info::disk_info().context("Failed to get disk info")?;
        let disk_space_free_gb = disk_info.free / (1024 * 1024); // Convert KB to GB
        
        // Check for NVIDIA GPU
        let has_nvidia_gpu = Command::new("nvidia-smi").output().map(|o| o.status.success()).unwrap_or(false);
        
        // Check for AMD GPU
        let has_amd_gpu = Command::new("rocm-smi").output().map(|o| o.status.success()).unwrap_or(false);
        
        // Get GPU info if available
        let gpu_info = if has_nvidia_gpu {
            self.detect_nvidia_gpu_info().ok()
        } else if has_amd_gpu {
            self.detect_amd_gpu_info().ok()
        } else {
            None
        };
        
        // Check network connectivity
        let network_info = self.check_network_connectivity()?;
        
        let system_info = SystemInfo {
            os_type,
            os_version,
            cpu_cores,
            memory_total_mb,
            disk_space_free_gb,
            has_nvidia_gpu,
            has_amd_gpu,
            gpu_info,
            network_info,
        };
        
        // Return the system info
        Ok(system_info)
    }
    
    /// Detect OS version
    fn detect_os_version(&self) -> Result<String> {
        match std::env::consts::OS {
            "windows" => {
                let output = Command::new("cmd")
                    .args(&["/c", "ver"])
                    .output()
                    .context("Failed to get Windows version")?;
                
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                
                Ok(version)
            },
            "linux" => {
                if let Ok(output) = Command::new("lsb_release")
                    .args(&["-ds"])
                    .output() {
                    
                    let version = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    
                    return Ok(version);
                }
                
                // Fallback to reading /etc/os-release
                if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                    for line in content.lines() {
                        if line.starts_with("PRETTY_NAME=") {
                            let version = line
                                .trim_start_matches("PRETTY_NAME=")
                                .trim_matches('"')
                                .to_string();
                            return Ok(version);
                        }
                    }
                }
                
                Ok("Unknown Linux".to_string())
            },
            "macos" => {
                let output = Command::new("sw_vers")
                    .args(&["-productVersion"])
                    .output()
                    .context("Failed to get macOS version")?;
                
                let version = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                
                Ok(format!("macOS {}", version))
            },
            os => Ok(format!("Unknown OS: {}", os)),
        }
    }
    
    /// Detect NVIDIA GPU information
    fn detect_nvidia_gpu_info(&self) -> Result<GpuInfo> {
        let output = Command::new("nvidia-smi")
            .args(&["--query-gpu=name,memory.total", "--format=csv,noheader"])
            .output()
            .context("Failed to get NVIDIA GPU info")?;
        
        let info_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = info_str.trim().split(',').collect();
        
        if parts.len() < 2 {
            return Err(anyhow!("Failed to parse NVIDIA GPU info"));
        }
        
        let model = parts[0].trim().to_string();
        let memory_str = parts[1].trim().replace(" MiB", "");
        let memory_mb = memory_str.parse::<u64>()
            .context("Failed to parse GPU memory")?;
        
        Ok(GpuInfo {
            gpu_type: "NVIDIA".to_string(),
            model,
            memory_mb,
        })
    }
    
    /// Detect AMD GPU information
    fn detect_amd_gpu_info(&self) -> Result<GpuInfo> {
        // This is a basic implementation - rocm-smi output format may vary
        // A real implementation would need to parse the actual output format
        
        let output = Command::new("rocm-smi")
            .args(&["--showproductname", "--showmeminfo", "vram"])
            .output()
            .context("Failed to get AMD GPU info")?;
        
        let info_str = String::from_utf8_lossy(&output.stdout);
        
        // Simplified parsing - would need to be adapted for actual rocm-smi output
        Ok(GpuInfo {
            gpu_type: "AMD".to_string(),
            model: "AMD GPU".to_string(), // Would extract actual model in real implementation
            memory_mb: 4096, // Would extract actual memory in real implementation
        })
    }
    
    /// Check network connectivity
    fn check_network_connectivity(&self) -> Result<NetworkInfo> {
        // Check internet connectivity by pinging a reliable server
        let ping_result = Command::new("ping")
            .args(match std::env::consts::OS {
                "windows" => &["-n", "1", "8.8.8.8"],
                _ => &["-c", "1", "8.8.8.8"],
            })
            .output();
        
        let internet_available = ping_result.map(|o| o.status.success()).unwrap_or(false);
        
        // We'll skip actual speed test for now as it requires more complex implementation
        Ok(NetworkInfo {
            internet_available,
            download_speed_mbps: None,
            upload_speed_mbps: None,
        })
    }
    
    /// Run system diagnostics
    pub fn run_diagnostics(&self) -> Result<HashMap<String, String>> {
        println!("Running system diagnostics...");
        
        let mut results = HashMap::new();
        
        // Detect system info
        let info = self.detect_system_info()?;
        
        // Add basic system info to results
        results.insert("os.type".to_string(), info.os_type);
        results.insert("os.version".to_string(), info.os_version);
        results.insert("cpu.cores".to_string(), info.cpu_cores.to_string());
        results.insert("memory.total_mb".to_string(), info.memory_total_mb.to_string());
        results.insert("disk.free_gb".to_string(), info.disk_space_free_gb.to_string());
        results.insert("gpu.nvidia".to_string(), info.has_nvidia_gpu.to_string());
        results.insert("gpu.amd".to_string(), info.has_amd_gpu.to_string());
        
        if let Some(gpu) = info.gpu_info {
            results.insert("gpu.type".to_string(), gpu.gpu_type);
            results.insert("gpu.model".to_string(), gpu.model);
            results.insert("gpu.memory_mb".to_string(), gpu.memory_mb.to_string());
        }
        
        results.insert("network.internet".to_string(), info.network_info.internet_available.to_string());
        
        // Run additional diagnostics in parallel
        let diagnostic_results: Vec<(String, String)> = vec![
            self.check_rust_environment(),
            self.check_python_environment(),
            self.check_cargo_environment(),
            self.check_openssl_environment(),
        ].into_par_iter()
         .filter_map(|res| res.ok())
         .collect();
         
        // Add all diagnostic results
        for (key, value) in diagnostic_results {
            results.insert(key, value);
        }
        
        println!("✓ System diagnostics completed");
        Ok(results)
    }
    
    /// Check Rust environment
    fn check_rust_environment(&self) -> Result<(String, String)> {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .context("Failed to get Rust version")?;
            
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
            
        Ok(("rust.version".to_string(), version))
    }
    
    /// Check Python environment
    fn check_python_environment(&self) -> Result<(String, String)> {
        // Try python3 first, then python
        let python_cmd = if Command::new("python3").arg("--version").output().is_ok() {
            "python3"
        } else {
            "python"
        };
        
        let output = Command::new(python_cmd)
            .arg("--version")
            .output()
            .context("Failed to get Python version")?;
            
        let version = if output.stdout.is_empty() {
            String::from_utf8_lossy(&output.stderr)
        } else {
            String::from_utf8_lossy(&output.stdout)
        }.trim().to_string();
            
        Ok(("python.version".to_string(), version))
    }
    
    /// Check Cargo environment
    fn check_cargo_environment(&self) -> Result<(String, String)> {
        let output = Command::new("cargo")
            .arg("--version")
            .output()
            .context("Failed to get Cargo version")?;
            
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
            
        Ok(("cargo.version".to_string(), version))
    }
    
    /// Check OpenSSL environment
    fn check_openssl_environment(&self) -> Result<(String, String)> {
        let cmd = match std::env::consts::OS {
            "windows" => "openssl",
            _ => "openssl",
        };
        
        let output = Command::new(cmd)
            .arg("version")
            .output()
            .context("Failed to get OpenSSL version")?;
            
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
            
        Ok(("openssl.version".to_string(), version))
    }
}
