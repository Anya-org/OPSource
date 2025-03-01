use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};

use crate::system::SystemInfo;
use crate::utils::format_bytes;

/// ML installer and configurator
pub struct MLManager {
    project_root: PathBuf,
    data_dir: PathBuf,
    system_info: Option<SystemInfo>,
    dry_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLConfig {
    pub enabled: bool,
    pub frameworks: Vec<String>,
    pub gpu_enabled: bool,
    pub optimized_for_memory: bool,
    pub batch_size: usize,
    pub num_threads: usize,
    pub precision: String,
}

impl MLManager {
    /// Create a new ML manager
    pub fn new(project_root: &Path, system_info: Option<SystemInfo>, dry_run: bool) -> Self {
        let data_dir = project_root.join("data").join("ml");
        
        Self {
            project_root: project_root.to_path_buf(),
            data_dir,
            system_info,
            dry_run,
        }
    }
    
    /// Install ML components
    pub fn install(&self, frameworks: &[String], auto_config: bool) -> Result<()> {
        println!("Installing ML components...");
        
        if self.dry_run {
            println!("Dry run: Would install ML frameworks: {:?}", frameworks);
            if auto_config {
                println!("Dry run: Would auto-configure ML settings based on system capabilities");
            }
            return Ok(());
        }
        
        // Ensure data directory exists
        if !self.data_dir.exists() {
            fs::create_dir_all(&self.data_dir)
                .context(format!("Failed to create ML data directory: {:?}", self.data_dir))?;
        }
        
        // Install Python if needed
        self.install_python()?;
        
        // Install frameworks
        for framework in frameworks {
            match framework.to_lowercase().as_str() {
                "tensorflow" => self.install_tensorflow()?,
                "pytorch" => self.install_pytorch()?,
                _ => println!("Unknown ML framework: {}", framework),
            }
        }
        
        // Auto-configure ML settings if requested
        if auto_config {
            self.auto_configure_ml(frameworks)?;
        }
        
        println!("✓ ML components installation completed");
        Ok(())
    }
    
    /// Install Python
    fn install_python(&self) -> Result<()> {
        if self.is_python_installed() {
            println!("✓ Python is already installed");
            return Ok(());
        }
        
        println!("Installing Python...");
        
        match std::env::consts::OS {
            "windows" => {
                // Download and install Python on Windows
                let python_installer = self.project_root.join("python_installer.exe");
                
                // Download Python installer
                let output = Command::new("powershell")
                    .args(&[
                        "-Command",
                        "(New-Object Net.WebClient).DownloadFile('https://www.python.org/ftp/python/3.10.0/python-3.10.0-amd64.exe', 'python_installer.exe')"
                    ])
                    .current_dir(&self.project_root)
                    .output()
                    .context("Failed to download Python installer")?;
                
                if !output.status.success() {
                    return Err(anyhow!("Failed to download Python installer"));
                }
                
                // Install Python
                let output = Command::new(python_installer)
                    .args(&["/quiet", "InstallAllUsers=1", "PrependPath=1"])
                    .output()
                    .context("Failed to run Python installer")?;
                
                if !output.status.success() {
                    return Err(anyhow!("Failed to install Python"));
                }
                
                // Clean up
                fs::remove_file(python_installer).ok();
            },
            "linux" => {
                // Install Python on Linux
                let package_manager = if Command::new("apt").arg("-v").output().map(|o| o.status.success()).unwrap_or(false) {
                    "apt"
                } else if Command::new("dnf").arg("-v").output().map(|o| o.status.success()).unwrap_or(false) {
                    "dnf"
                } else if Command::new("yum").arg("-v").output().map(|o| o.status.success()).unwrap_or(false) {
                    "yum"
                } else {
                    return Err(anyhow!("Unsupported Linux distribution"));
                };
                
                let (cmd, args) = match package_manager {
                    "apt" => ("apt", vec!["install", "-y", "python3", "python3-pip", "python3-venv"]),
                    "dnf" | "yum" => (package_manager, vec!["install", "-y", "python3", "python3-pip"]),
                    _ => return Err(anyhow!("Unsupported package manager")),
                };
                
                let output = Command::new(cmd)
                    .args(&args)
                    .output()
                    .context(format!("Failed to install Python using {}", package_manager))?;
                
                if !output.status.success() {
                    return Err(anyhow!("Failed to install Python"));
                }
            },
            "macos" => {
                // Install Python on macOS using Homebrew
                if !Command::new("brew").arg("-v").output().map(|o| o.status.success()).unwrap_or(false) {
                    return Err(anyhow!("Homebrew is required to install Python on macOS"));
                }
                
                let output = Command::new("brew")
                    .args(&["install", "python@3.10"])
                    .output()
                    .context("Failed to install Python using Homebrew")?;
                
                if !output.status.success() {
                    return Err(anyhow!("Failed to install Python"));
                }
            },
            _ => return Err(anyhow!("Unsupported operating system")),
        }
        
        println!("✓ Python installed successfully");
        Ok(())
    }
    
    /// Check if Python is installed
    fn is_python_installed(&self) -> bool {
        // Try python3 first, then python
        let python_cmd = if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python3"
        } else if Command::new("python").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python"
        } else {
            return false;
        };
        
        // Check if pip is available
        Command::new(python_cmd)
            .args(&["-m", "pip", "--version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    /// Install TensorFlow
    fn install_tensorflow(&self) -> Result<()> {
        println!("Installing TensorFlow...");
        
        let python_cmd = if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python3"
        } else {
            "python"
        };
        
        // Check if system has GPU support
        let has_gpu = if let Some(info) = &self.system_info {
            info.has_nvidia_gpu || info.has_amd_gpu
        } else {
            false
        };
        
        // Install TensorFlow with or without GPU support
        let tf_package = if has_gpu { "tensorflow" } else { "tensorflow-cpu" };
        
        let output = Command::new(python_cmd)
            .args(&["-m", "pip", "install", "--upgrade", tf_package])
            .output()
            .context("Failed to install TensorFlow")?;
        
        if !output.status.success() {
            return Err(anyhow!("Failed to install TensorFlow: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("✓ TensorFlow installed successfully");
        Ok(())
    }
    
    /// Install PyTorch
    fn install_pytorch(&self) -> Result<()> {
        println!("Installing PyTorch...");
        
        let python_cmd = if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python3"
        } else {
            "python"
        };
        
        // Check if system has GPU support
        let has_gpu = if let Some(info) = &self.system_info {
            info.has_nvidia_gpu || info.has_amd_gpu
        } else {
            false
        };
        
        // Install PyTorch with or without GPU support
        let pytorch_package = if has_gpu {
            if self.system_info.as_ref().map_or(false, |info| info.has_nvidia_gpu) {
                "torch torchvision torchaudio"
            } else {
                "torch torchvision torchaudio --index-url https://download.pytorch.org/whl/rocm5.4.2"
            }
        } else {
            "torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu"
        };
        
        let output = Command::new(python_cmd)
            .args(&["-m", "pip", "install", "--upgrade"])
            .args(pytorch_package.split_whitespace())
            .output()
            .context("Failed to install PyTorch")?;
        
        if !output.status.success() {
            return Err(anyhow!("Failed to install PyTorch: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("✓ PyTorch installed successfully");
        Ok(())
    }
    
    /// Auto-configure ML settings based on system capabilities
    fn auto_configure_ml(&self, frameworks: &[String]) -> Result<()> {
        println!("Auto-configuring ML settings...");
        
        // Get system info
        let system_info = match &self.system_info {
            Some(info) => info,
            None => return Err(anyhow!("System information not available for ML auto-configuration")),
        };
        
        // Calculate optimal batch size based on available memory
        let memory_mb = system_info.memory_total_mb;
        let cpu_cores = system_info.cpu_cores;
        let has_gpu = system_info.has_nvidia_gpu || system_info.has_amd_gpu;
        
        let batch_size = if memory_mb > 16384 {
            64  // 16GB+ RAM
        } else if memory_mb > 8192 {
            32  // 8GB+ RAM
        } else if memory_mb > 4096 {
            16  // 4GB+ RAM
        } else {
            8   // Less than 4GB RAM
        };
        
        // Configure number of threads based on CPU cores
        let num_threads = if cpu_cores > 8 {
            cpu_cores - 2  // Leave some cores for system
        } else if cpu_cores > 4 {
            cpu_cores - 1
        } else {
            cpu_cores
        };
        
        // Create ML configuration
        let ml_config = MLConfig {
            enabled: true,
            frameworks: frameworks.to_vec(),
            gpu_enabled: has_gpu,
            optimized_for_memory: memory_mb < 8192,
            batch_size,
            num_threads: num_threads as usize,
            precision: if has_gpu && memory_mb > 8192 { "float32".to_string() } else { "float16".to_string() },
        };
        
        // Create config directory if it doesn't exist
        let config_dir = self.project_root.join("config").join("ml");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context(format!("Failed to create ML config directory: {:?}", config_dir))?;
        }
        
        // Write ML configuration to file
        let config_path = config_dir.join("ml_config.json");
        let config_json = serde_json::to_string_pretty(&ml_config)
            .context("Failed to serialize ML configuration")?;
        
        fs::write(&config_path, config_json)
            .context(format!("Failed to write ML configuration to {:?}", config_path))?;
        
        // Generate configuration files for each framework
        for framework in frameworks {
            match framework.to_lowercase().as_str() {
                "tensorflow" => self.generate_tensorflow_config(&ml_config, &config_dir)?,
                "pytorch" => self.generate_pytorch_config(&ml_config, &config_dir)?,
                _ => println!("Skipping unknown framework: {}", framework),
            }
        }
        
        println!("✓ ML auto-configuration completed");
        println!("  - Batch size: {}", batch_size);
        println!("  - Threads: {}", num_threads);
        println!("  - GPU enabled: {}", has_gpu);
        println!("  - Memory optimization: {}", if memory_mb < 8192 { "enabled" } else { "disabled" });
        
        Ok(())
    }
    
    /// Generate TensorFlow configuration
    fn generate_tensorflow_config(&self, ml_config: &MLConfig, config_dir: &Path) -> Result<()> {
        println!("Generating TensorFlow configuration...");
        
        let tf_config = format!(
            r#"import tensorflow as tf

# Auto-generated TensorFlow configuration
def configure_tensorflow():
    """Configure TensorFlow based on system capabilities."""
    # Set the number of threads
    tf.config.threading.set_intra_op_parallelism_threads({threads})
    tf.config.threading.set_inter_op_parallelism_threads({threads})
    
    # Memory growth for GPUs
    gpus = tf.config.experimental.list_physical_devices('GPU')
    if gpus:
        try:
            for gpu in gpus:
                tf.config.experimental.set_memory_growth(gpu, True)
            print(f"GPU memory growth enabled for {{len(gpus)}} GPUs")
        except RuntimeError as e:
            print(f"Error setting GPU memory growth: {{e}}")
    
    # Set precision
    if '{precision}' == 'float16' and gpus:
        tf.keras.mixed_precision.set_global_policy('mixed_float16')
        print("Using mixed precision (float16)")
    
    print("TensorFlow configured successfully")

# Set default batch size
DEFAULT_BATCH_SIZE = {batch_size}
"#,
            threads = ml_config.num_threads,
            precision = ml_config.precision,
            batch_size = ml_config.batch_size
        );
        
        // Write configuration to file
        let tf_config_path = config_dir.join("tensorflow_config.py");
        fs::write(&tf_config_path, tf_config)
            .context(format!("Failed to write TensorFlow configuration to {:?}", tf_config_path))?;
        
        println!("✓ TensorFlow configuration saved to {:?}", tf_config_path);
        Ok(())
    }
    
    /// Generate PyTorch configuration
    fn generate_pytorch_config(&self, ml_config: &MLConfig, config_dir: &Path) -> Result<()> {
        println!("Generating PyTorch configuration...");
        
        let torch_config = format!(
            r#"import torch
import os

# Auto-generated PyTorch configuration
def configure_pytorch():
    """Configure PyTorch based on system capabilities."""
    # Set the number of threads
    torch.set_num_threads({threads})
    
    # Configure GPU if available
    if torch.cuda.is_available():
        print(f"GPU available: {{torch.cuda.get_device_name(0)}}")
        
        # Set precision
        if '{precision}' == 'float16':
            # Enable automatic mixed precision
            print("Using automatic mixed precision (float16)")
            
    else:
        print("GPU not available, using CPU")
    
    print("PyTorch configured successfully")

# Set default batch size
DEFAULT_BATCH_SIZE = {batch_size}
"#,
            threads = ml_config.num_threads,
            precision = ml_config.precision,
            batch_size = ml_config.batch_size
        );
        
        // Write configuration to file
        let torch_config_path = config_dir.join("pytorch_config.py");
        fs::write(&torch_config_path, torch_config)
            .context(format!("Failed to write PyTorch configuration to {:?}", torch_config_path))?;
        
        println!("✓ PyTorch configuration saved to {:?}", torch_config_path);
        Ok(())
    }
    
    /// Test ML installation
    pub fn test(&self, json_output: bool) -> Result<()> {
        println!("Testing ML installation...");
        
        if self.dry_run {
            println!("Dry run: Would test ML installation");
            return Ok(());
        }
        
        // Check if ML is installed
        let config_dir = self.project_root.join("config").join("ml");
        let ml_config_path = config_dir.join("ml_config.json");
        
        if !ml_config_path.exists() {
            println!("ML not installed. Skipping ML tests.");
            return Ok(());
        }
        
        // Load ML configuration
        let config_str = fs::read_to_string(&ml_config_path)
            .context(format!("Failed to read ML configuration from {:?}", ml_config_path))?;
            
        let ml_config: MLConfig = serde_json::from_str(&config_str)
            .context("Failed to parse ML configuration")?;
        
        // Test each installed framework
        for framework in &ml_config.frameworks {
            match framework.to_lowercase().as_str() {
                "tensorflow" => self.test_tensorflow(json_output)?,
                "pytorch" => self.test_pytorch(json_output)?,
                _ => println!("Skipping test for unknown framework: {}", framework),
            }
        }
        
        println!("✓ ML tests completed successfully");
        Ok(())
    }
    
    /// Test TensorFlow installation
    fn test_tensorflow(&self, json_output: bool) -> Result<()> {
        println!("Testing TensorFlow installation...");
        
        let python_cmd = if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python3"
        } else {
            "python"
        };
        
        // Create a simple test script
        let test_script = r#"
import tensorflow as tf
import json
import sys

# Test TensorFlow installation
def test_tensorflow():
    try:
        # Print TensorFlow version
        print(f"TensorFlow version: {tf.__version__}")
        
        # Check if GPU is available
        gpu_available = len(tf.config.list_physical_devices('GPU')) > 0
        print(f"GPU available: {gpu_available}")
        
        # Create a simple tensor and perform an operation
        a = tf.constant([[1.0, 2.0], [3.0, 4.0]])
        b = tf.constant([[5.0, 6.0], [7.0, 8.0]])
        c = tf.matmul(a, b)
        
        # Convert result to a list
        result = c.numpy().tolist()
        
        return {
            "success": True,
            "version": tf.__version__,
            "gpu_available": gpu_available,
            "computation_result": result
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e)
        }

# Run test and print result
result = test_tensorflow()
if len(sys.argv) > 1 and sys.argv[1] == "--json":
    print(json.dumps(result, indent=2))
else:
    if result["success"]:
        print("✓ TensorFlow test successful")
    else:
        print(f"✗ TensorFlow test failed: {result['error']}")
"#;
        
        // Write the test script to a temporary file
        let test_script_path = self.data_dir.join("test_tensorflow.py");
        fs::write(&test_script_path, test_script)
            .context(format!("Failed to write TensorFlow test script to {:?}", test_script_path))?;
        
        // Run the test script
        let mut args = vec![test_script_path.to_string_lossy().to_string()];
        if json_output {
            args.push("--json".to_string());
        }
        
        let output = Command::new(python_cmd)
            .args(&args)
            .output()
            .context("Failed to run TensorFlow test script")?;
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
        
        if !output.status.success() {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            return Err(anyhow!("TensorFlow test failed"));
        }
        
        Ok(())
    }
    
    /// Test PyTorch installation
    fn test_pytorch(&self, json_output: bool) -> Result<()> {
        println!("Testing PyTorch installation...");
        
        let python_cmd = if Command::new("python3").arg("--version").output().map(|o| o.status.success()).unwrap_or(false) {
            "python3"
        } else {
            "python"
        };
        
        // Create a simple test script
        let test_script = r#"
import torch
import json
import sys

# Test PyTorch installation
def test_pytorch():
    try:
        # Print PyTorch version
        print(f"PyTorch version: {torch.__version__}")
        
        # Check if CUDA is available
        cuda_available = torch.cuda.is_available()
        print(f"CUDA available: {cuda_available}")
        
        # Create a simple tensor and perform an operation
        a = torch.tensor([[1.0, 2.0], [3.0, 4.0]])
        b = torch.tensor([[5.0, 6.0], [7.0, 8.0]])
        c = torch.matmul(a, b)
        
        # Convert result to a list
        result = c.tolist()
        
        return {
            "success": True,
            "version": torch.__version__,
            "cuda_available": cuda_available,
            "computation_result": result
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e)
        }

# Run test and print result
result = test_pytorch()
if len(sys.argv) > 1 and sys.argv[1] == "--json":
    print(json.dumps(result, indent=2))
else:
    if result["success"]:
        print("✓ PyTorch test successful")
    else:
        print(f"✗ PyTorch test failed: {result['error']}")
"#;
        
        // Write the test script to a temporary file
        let test_script_path = self.data_dir.join("test_pytorch.py");
        fs::write(&test_script_path, test_script)
            .context(format!("Failed to write PyTorch test script to {:?}", test_script_path))?;
        
        // Run the test script
        let mut args = vec![test_script_path.to_string_lossy().to_string()];
        if json_output {
            args.push("--json".to_string());
        }
        
        let output = Command::new(python_cmd)
            .args(&args)
            .output()
            .context("Failed to run PyTorch test script")?;
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
        
        if !output.status.success() {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            return Err(anyhow!("PyTorch test failed"));
        }
        
        Ok(())
    }
}
