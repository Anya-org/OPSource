use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use anyhow::{Result, Context, anyhow};

use crate::config::ConfigManager;
use crate::utils::{is_command_available, get_command_path, should_install_component};

/// OPSource installer
pub struct OPSourceInstaller {
    config_manager: ConfigManager,
    project_root: PathBuf,
    dry_run: bool,
}

impl OPSourceInstaller {
    /// Create a new OPSource installer
    pub fn new(config_manager: ConfigManager, project_root: &Path, dry_run: bool) -> Self {
        Self {
            config_manager,
            project_root: project_root.to_path_buf(),
            dry_run,
        }
    }
    
    /// Install OPSource
    pub fn install(&self, component: Option<&str>) -> Result<()> {
        if !should_install_component(component, "opsource") {
            return Ok(());
        }
        
        println!("Installing OPSource...");
        
        // Ensure Rust is installed
        self.check_rust_installation()?;
        
        // Install dependencies
        self.install_dependencies()?;
        
        // Build OPSource
        self.build_opsource()?;
        
        // Configure OPSource
        self.configure_opsource()?;
        
        println!("✓ OPSource installation completed");
        Ok(())
    }
    
    /// Check if Rust is installed and install it if necessary
    fn check_rust_installation(&self) -> Result<()> {
        println!("Checking Rust installation...");
        
        if !is_command_available("rustc") || !is_command_available("cargo") {
            println!("Rust is not installed. Installing...");
            
            if self.dry_run {
                println!("Dry run: Would install Rust using rustup");
                return Ok(());
            }
            
            // Install Rust using rustup
            let rustup_url = "https://sh.rustup.rs";
            
            match std::env::consts::OS {
                "windows" => {
                    // On Windows, download and run rustup-init.exe
                    let rustup_init = self.project_root.join("rustup-init.exe");
                    
                    // Download rustup-init.exe
                    let output = Command::new("powershell")
                        .args(&[
                            "-Command",
                            &format!(
                                "(New-Object Net.WebClient).DownloadFile('https://win.rustup.rs/x86_64', '{}')",
                                rustup_init.display()
                            ),
                        ])
                        .output()
                        .context("Failed to download rustup-init.exe")?;
                        
                    if !output.status.success() {
                        return Err(anyhow!("Failed to download rustup-init.exe: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                    }
                    
                    // Run rustup-init.exe
                    let output = Command::new(&rustup_init)
                        .args(&["-y", "--default-toolchain", "stable", "--profile", "minimal"])
                        .output()
                        .context("Failed to run rustup-init.exe")?;
                        
                    if !output.status.success() {
                        return Err(anyhow!("Failed to install Rust: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                    }
                    
                    // Clean up
                    if rustup_init.exists() {
                        fs::remove_file(rustup_init).ok();
                    }
                },
                _ => {
                    // On Unix systems, use curl | sh
                    let output = Command::new("curl")
                        .args(&["--proto", "=https", "--tlsv1.2", "-sSf", rustup_url])
                        .stdout(std::process::Stdio::piped())
                        .spawn()
                        .context("Failed to download rustup")?;
                        
                    let rustup_installer = output.stdout
                        .context("Failed to get rustup installer output")?;
                        
                    let output = Command::new("sh")
                        .args(&["-s", "--", "-y", "--default-toolchain", "stable", "--profile", "minimal"])
                        .stdin(rustup_installer)
                        .output()
                        .context("Failed to run rustup installer")?;
                        
                    if !output.status.success() {
                        return Err(anyhow!("Failed to install Rust: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                    }
                }
            }
            
            println!("✓ Rust installation completed");
        } else {
            println!("✓ Rust is already installed");
        }
        
        Ok(())
    }
    
    /// Install OPSource dependencies
    fn install_dependencies(&self) -> Result<()> {
        println!("Installing OPSource dependencies...");
        
        if self.dry_run {
            println!("Dry run: Would install OPSource dependencies");
            return Ok(());
        }
        
        // Install system-level dependencies
        match std::env::consts::OS {
            "windows" => {
                // On Windows, many dependencies will be handled by Cargo
                // But we may need some specific dependencies
                if !is_command_available("openssl") {
                    println!("Installing OpenSSL using vcpkg...");
                    
                    // Check if vcpkg is installed
                    if !is_command_available("vcpkg") {
                        return Err(anyhow!("vcpkg not found. Please install vcpkg to install OpenSSL."));
                    }
                    
                    // Install OpenSSL using vcpkg
                    let output = Command::new("vcpkg")
                        .args(&["install", "openssl:x64-windows"])
                        .output()
                        .context("Failed to install OpenSSL using vcpkg")?;
                        
                    if !output.status.success() {
                        return Err(anyhow!("Failed to install OpenSSL: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                    }
                    
                    println!("✓ OpenSSL installed");
                }
            },
            "linux" => {
                // Install dependencies for Linux
                // This will vary based on the distribution
                let package_manager = if is_command_available("apt") {
                    "apt"
                } else if is_command_available("dnf") {
                    "dnf"
                } else if is_command_available("yum") {
                    "yum"
                } else if is_command_available("pacman") {
                    "pacman"
                } else {
                    return Err(anyhow!("Unsupported Linux distribution. Please install dependencies manually."));
                };
                
                println!("Installing system dependencies using {}...", package_manager);
                
                let (cmd, args) = match package_manager {
                    "apt" => ("apt", vec!["install", "-y", "build-essential", "pkg-config", "libssl-dev"]),
                    "dnf" | "yum" => (package_manager, vec!["install", "-y", "gcc", "openssl-devel"]),
                    "pacman" => ("pacman", vec!["-Sy", "--noconfirm", "base-devel", "openssl"]),
                    _ => return Err(anyhow!("Unsupported package manager")),
                };
                
                let output = Command::new(cmd)
                    .args(&args)
                    .output()
                    .context(format!("Failed to install dependencies using {}", package_manager))?;
                    
                if !output.status.success() {
                    return Err(anyhow!("Failed to install dependencies: {}", 
                        String::from_utf8_lossy(&output.stderr)));
                }
                
                println!("✓ System dependencies installed");
            },
            "macos" => {
                // Install dependencies for macOS
                if !is_command_available("brew") {
                    return Err(anyhow!("Homebrew not found. Please install Homebrew to install dependencies."));
                }
                
                println!("Installing system dependencies using Homebrew...");
                
                let output = Command::new("brew")
                    .args(&["install", "openssl"])
                    .output()
                    .context("Failed to install dependencies using Homebrew")?;
                    
                if !output.status.success() {
                    return Err(anyhow!("Failed to install dependencies: {}", 
                        String::from_utf8_lossy(&output.stderr)));
                }
                
                println!("✓ System dependencies installed");
            },
            os => return Err(anyhow!("Unsupported operating system: {}", os)),
        }
        
        println!("✓ OPSource dependencies installed");
        Ok(())
    }
    
    /// Build OPSource from source
    fn build_opsource(&self) -> Result<()> {
        println!("Building OPSource...");
        
        if self.dry_run {
            println!("Dry run: Would build OPSource using cargo");
            return Ok(());
        }
        
        // Get the path to cargo
        let cargo_path = if let Ok(path) = get_command_path("cargo") {
            path
        } else {
            // If cargo is not found, try to find it in the rustup directory
            match std::env::consts::OS {
                "windows" => PathBuf::from(r"C:\Users\\.cargo\bin\cargo.exe"),
                _ => PathBuf::from("~/.cargo/bin/cargo"),
            }
        };
        
        // Build OPSource using cargo
        let output = Command::new(cargo_path)
            .current_dir(&self.project_root)
            .args(&["build", "--release"])
            .output()
            .context("Failed to build OPSource")?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to build OPSource: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("✓ OPSource built successfully");
        Ok(())
    }
    
    /// Configure OPSource
    fn configure_opsource(&self) -> Result<()> {
        println!("Configuring OPSource...");
        
        if self.dry_run {
            println!("Dry run: Would configure OPSource");
            return Ok(());
        }
        
        // Generate OPSource configuration
        self.config_manager.generate_bitcoin_conf()?;
        
        println!("✓ OPSource configured successfully");
        Ok(())
    }
    
    /// Uninstall OPSource
    pub fn uninstall(&self) -> Result<()> {
        println!("Uninstalling OPSource...");
        
        if self.dry_run {
            println!("Dry run: Would uninstall OPSource");
            return Ok(());
        }
        
        // Remove OPSource binary
        let bin_dir = self.project_root.join("target").join("release");
        let executable_name = if cfg!(windows) { "opsource.exe" } else { "opsource" };
        let executable_path = bin_dir.join(executable_name);
        
        if executable_path.exists() {
            fs::remove_file(&executable_path)
                .context(format!("Failed to remove OPSource executable: {:?}", executable_path))?;
                
            println!("✓ OPSource executable removed");
        }
        
        // Remove configuration files
        let config_dir = self.project_root.join("config");
        if config_dir.exists() {
            fs::remove_dir_all(&config_dir)
                .context(format!("Failed to remove OPSource configuration directory: {:?}", config_dir))?;
                
            println!("✓ OPSource configuration removed");
        }
        
        println!("✓ OPSource uninstallation completed");
        Ok(())
    }
    
    /// Run OPSource tests
    pub fn test(&self) -> Result<()> {
        println!("Running OPSource tests...");
        
        if self.dry_run {
            println!("Dry run: Would run OPSource tests");
            return Ok(());
        }
        
        // Get the path to cargo
        let cargo_path = if let Ok(path) = get_command_path("cargo") {
            path
        } else {
            // If cargo is not found, try to find it in the rustup directory
            match std::env::consts::OS {
                "windows" => PathBuf::from(r"C:\Users\\.cargo\bin\cargo.exe"),
                _ => PathBuf::from("~/.cargo/bin/cargo"),
            }
        };
        
        // Run OPSource tests using cargo
        let output = Command::new(cargo_path)
            .current_dir(&self.project_root)
            .args(&["test"])
            .output()
            .context("Failed to run OPSource tests")?;
            
        if !output.status.success() {
            return Err(anyhow!("Some OPSource tests failed: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("✓ All OPSource tests passed");
        Ok(())
    }
}
