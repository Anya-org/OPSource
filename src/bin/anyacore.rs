use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use anyhow::{Result, Context, anyhow};

use crate::config::{ConfigManager, AnyaConfig, AnyaBitcoinConfig, AnyaLightningConfig, AnyaWeb5Config, AnyaExtensionsConfig};
use crate::utils::{is_command_available, get_command_path, should_install_component};

/// Anya Core installer
pub struct AnyaCoreInstaller {
    config_manager: ConfigManager,
    project_root: PathBuf,
    anya_root: PathBuf,
    dry_run: bool,
}

impl AnyaCoreInstaller {
    /// Create a new Anya Core installer
    pub fn new(config_manager: ConfigManager, project_root: &Path, dry_run: bool) -> Result<Self> {
        let anya_root = project_root.join("anya-core");
        
        if !anya_root.exists() {
            return Err(anyhow!("Anya Core directory not found: {:?}", anya_root));
        }
        
        Ok(Self {
            config_manager,
            project_root: project_root.to_path_buf(),
            anya_root,
            dry_run,
        })
    }
    
    /// Install Anya Core
    pub fn install(&self, component: Option<&str>, with_lightning: bool, with_web5: bool) -> Result<()> {
        if !should_install_component(component, "anyacore") {
            return Ok(());
        }
        
        println!("Installing Anya Core...");
        
        // Ensure Rust is installed
        self.check_rust_installation()?;
        
        // Install dependencies
        self.install_dependencies(with_lightning, with_web5)?;
        
        // Build Anya Core
        self.build_anyacore(with_lightning, with_web5)?;
        
        // Configure Anya Core
        self.configure_anyacore(with_lightning, with_web5)?;
        
        println!("✓ Anya Core installation completed");
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
    
    /// Install Anya Core dependencies
    fn install_dependencies(&self, with_lightning: bool, with_web5: bool) -> Result<()> {
        println!("Installing Anya Core dependencies...");
        
        if self.dry_run {
            println!("Dry run: Would install Anya Core dependencies");
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
                
                let mut packages = vec!["build-essential", "pkg-config", "libssl-dev"];
                
                // Add Lightning dependencies if needed
                if with_lightning {
                    packages.push("libsqlite3-dev");
                }
                
                // Add Web5 dependencies if needed
                if with_web5 {
                    packages.push("libudev-dev");
                }
                
                let (cmd, args) = match package_manager {
                    "apt" => {
                        let mut apt_args = vec!["install", "-y"];
                        apt_args.extend(packages);
                        ("apt", apt_args)
                    },
                    "dnf" | "yum" => {
                        let mut dnf_args = vec!["install", "-y", "gcc", "openssl-devel"];
                        if with_lightning { dnf_args.push("sqlite-devel"); }
                        if with_web5 { dnf_args.push("libudev-devel"); }
                        (package_manager, dnf_args)
                    },
                    "pacman" => {
                        let mut pacman_args = vec!["-Sy", "--noconfirm", "base-devel", "openssl"];
                        if with_lightning { pacman_args.push("sqlite"); }
                        if with_web5 { pacman_args.push("libudev"); }
                        ("pacman", pacman_args)
                    },
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
                
                let mut packages = vec!["openssl"];
                
                // Add Lightning dependencies if needed
                if with_lightning {
                    packages.push("sqlite3");
                }
                
                // Add Web5 dependencies if needed
                if with_web5 {
                    packages.push("libudev-zero");
                }
                
                let output = Command::new("brew")
                    .arg("install")
                    .args(packages)
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
        
        println!("✓ Anya Core dependencies installed");
        Ok(())
    }
    
    /// Build Anya Core from source
    fn build_anyacore(&self, with_lightning: bool, with_web5: bool) -> Result<()> {
        println!("Building Anya Core...");
        
        if self.dry_run {
            println!("Dry run: Would build Anya Core using cargo");
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
        
        // Prepare feature flags
        let mut features = Vec::new();
        if with_lightning {
            features.push("lightning");
        }
        if with_web5 {
            features.push("web5");
        }
        
        // Build Anya Core using cargo
        let mut cmd = Command::new(cargo_path);
        cmd.current_dir(&self.anya_root)
            .args(&["build", "--release"]);
            
        // Add features if specified
        if !features.is_empty() {
            cmd.args(&["--features", &features.join(",")]);
        }
        
        let output = cmd.output()
            .context("Failed to build Anya Core")?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to build Anya Core: {}", 
                String::from_utf8_lossy(&output.stderr)));
        }
        
        println!("✓ Anya Core built successfully");
        Ok(())
    }
    
    /// Configure Anya Core
    fn configure_anyacore(&self, with_lightning: bool, with_web5: bool) -> Result<()> {
        println!("Configuring Anya Core...");
        
        if self.dry_run {
            println!("Dry run: Would configure Anya Core");
            return Ok(());
        }
        
        // Get config from ConfigManager
        let config = self.config_manager.get_config();
        
        // Create Anya Config
        let anya_config = AnyaConfig {
            bitcoin: AnyaBitcoinConfig {
                network: config.bitcoin_network.clone(),
                rpc_user: config.bitcoin_rpc_user.clone(),
                rpc_password: config.bitcoin_rpc_password.clone(),
                rpc_port: config.bitcoin_rpc_port,
                data_dir: format!("{}/bitcoin", config.data_dir),
            },
            lightning: if with_lightning {
                Some(AnyaLightningConfig {
                    enabled: true,
                    implementation: "ldk".to_string(),
                    network: config.bitcoin_network.clone(),
                    data_dir: format!("{}/lightning", config.data_dir),
                })
            } else {
                None
            },
            web5: if with_web5 {
                Some(AnyaWeb5Config {
                    enabled: true,
                    did_method: "key".to_string(),
                    port: 3000,
                })
            } else {
                None
            },
            extensions: Some(AnyaExtensionsConfig {
                enabled: true,
                extensions_dir: format!("{}/extensions", config.data_dir),
                enabled_extensions: Vec::new(),
            }),
        };
        
        // Create the config directory
        let config_dir = self.anya_root.join("config");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context(format!("Failed to create Anya Core config directory: {:?}", config_dir))?;
        }
        
        // Write config file
        let config_path = config_dir.join("config.json");
        let config_json = serde_json::to_string_pretty(&anya_config)
            .context("Failed to serialize Anya Core config")?;
            
        fs::write(&config_path, config_json)
            .context(format!("Failed to write Anya Core config to {:?}", config_path))?;
            
        println!("✓ Anya Core configuration saved to {:?}", config_path);
        Ok(())
    }
    
    /// Uninstall Anya Core
    pub fn uninstall(&self) -> Result<()> {
        println!("Uninstalling Anya Core...");
        
        if self.dry_run {
            println!("Dry run: Would uninstall Anya Core");
            return Ok(());
        }
        
        // Remove Anya Core binary
        let bin_dir = self.anya_root.join("target").join("release");
        let executable_name = if cfg!(windows) { "anya.exe" } else { "anya" };
        let executable_path = bin_dir.join(executable_name);
        
        if executable_path.exists() {
            fs::remove_file(&executable_path)
                .context(format!("Failed to remove Anya Core executable: {:?}", executable_path))?;
                
            println!("✓ Anya Core executable removed");
        }
        
        // Remove configuration files
        let config_dir = self.anya_root.join("config");
        if config_dir.exists() {
            fs::remove_dir_all(&config_dir)
                .context(format!("Failed to remove Anya Core configuration directory: {:?}", config_dir))?;
                
            println!("✓ Anya Core configuration removed");
        }
        
        println!("✓ Anya Core uninstallation completed");
        Ok(())
    }
    
    /// Run Anya Core tests
    pub fn test(&self, component: Option<&str>, json_output: bool) -> Result<()> {
        println!("Running Anya Core tests...");
        
        if self.dry_run {
            println!("Dry run: Would run Anya Core tests");
            if let Some(comp) = component {
                println!("  - Component: {}", comp);
            }
            if json_output {
                println!("  - JSON output: enabled");
            }
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
        
        // Prepare test arguments
        let mut args = vec!["test"];
        
        // Add specific test if component is specified
        if let Some(comp) = component {
            match comp {
                "bitcoin" => args.push("--package=anya-bitcoin"),
                "lightning" => args.push("--package=lightning"),
                "web5" => args.push("--package=web5"),
                "extensions" => args.push("--package=anya-extensions"),
                _ => {}, // Run all tests
            }
        }
        
        // Run Anya Core tests using cargo
        let output = Command::new(cargo_path)
            .current_dir(&self.anya_root)
            .args(&args)
            .output()
            .context("Failed to run Anya Core tests")?;
        
        // Format the output based on the json_output flag
        if json_output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            let test_results = serde_json::json!({
                "success": output.status.success(),
                "component": component,
                "stdout": stdout,
                "stderr": stderr
            });
            
            println!("{}", serde_json::to_string_pretty(&test_results).unwrap());
        } else {
            // Print regular output
            println!("{}", String::from_utf8_lossy(&output.stdout));
            
            if !output.stdout.is_empty() && !output.stderr.is_empty() {
                println!("\nErrors:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        if !output.status.success() {
            return Err(anyhow!("Some Anya Core tests failed"));
        }
        
        println!("✓ All Anya Core tests passed");
        Ok(())
    }
}
