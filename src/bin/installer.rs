use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Result, Context, anyhow};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

mod opsource;
mod anyacore;
mod ml;
mod utils;
mod system;
mod config;
mod wallet;

use crate::opsource::OPSourceInstaller;
use crate::anyacore::AnyaCoreInstaller;
use crate::ml::MLManager;
use crate::system::SystemDetector;
use crate::config::ConfigManager;
use crate::utils::*;
use crate::wallet::WalletManager;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(
    name = "OPSource Unified Installer",
    about = "Unified installer for OPSource and anya-core",
    version = VERSION,
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    
    /// Run commands in dry-run mode
    #[clap(long)]
    dry_run: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Install OPSource and/or anya-core
    Install {
        /// Install core components only
        #[clap(long)]
        core_only: bool,
        
        /// Skip confirmation prompts
        #[clap(short, long)]
        yes: bool,
        
        /// Install Python dependencies
        #[clap(long)]
        with_python: bool,
        
        /// Specific component to install (bitcoin, web5, rgb, dlc, taproot, all)
        #[clap(long)]
        component: Option<String>,
        
        /// Install OPSource components
        #[clap(long)]
        opsource: bool,
        
        /// Install anya-core components
        #[clap(long)]
        anya: bool,
        
        /// Anya-core modules to install (comma-separated: bitcoin,lightning,web5,extensions)
        #[clap(long)]
        anya_modules: Option<String>,
        
        /// Auto-configure anya-core after installation
        #[clap(long)]
        auto_config_anya: bool,
        
        /// Install machine learning components
        #[clap(long)]
        with_ml: bool,
        
        /// ML framework to install (tensorflow, pytorch, both)
        #[clap(long)]
        ml_framework: Option<String>,
        
        /// Auto-configure ML based on machine specs
        #[clap(long)]
        auto_config_ml: bool,

        /// Maximum memory to use during installation (in MB)
        #[clap(long)]
        max_memory: Option<u64>,
        
        /// Setup wallet during installation
        #[clap(long)]
        setup_wallet: bool,
        
        /// Setup DAO during installation
        #[clap(long)]
        setup_dao: bool,
    },
    
    /// Test installation and create report
    Test {
        /// Test specific component (bitcoin, web5, rgb, dlc, taproot, all)
        #[clap(long)]
        component: Option<String>,
        
        /// Generate JSON report
        #[clap(long)]
        json: bool,
        
        /// Maximum memory to use during tests (in MB)
        #[clap(long)]
        max_memory: Option<u64>,
    },
    
    /// Configure application
    Configure {
        /// Configuration file path
        #[clap(long)]
        config: Option<String>,
        
        /// Set specific configuration value (format: key=value)
        #[clap(long)]
        set: Vec<String>,
        
        /// Show current configuration
        #[clap(long)]
        show: bool,
    },
}

/// Main installer responsible for coordinating the installation process
pub struct Installer {
    project_root: PathBuf,
    dry_run: bool,
    config_manager: Arc<ConfigManager>,
    system_detector: SystemDetector,
    max_memory: Option<u64>,
}

impl Installer {
    /// Create a new installer instance
    fn new(dry_run: bool, max_memory: Option<u64>) -> Result<Self> {
        let project_root = find_project_root()?;
        let config_manager = Arc::new(ConfigManager::new(&project_root)?);
        let system_detector = SystemDetector::new();
        
        Ok(Self {
            project_root,
            dry_run,
            config_manager,
            system_detector,
            max_memory,
        })
    }
    
    /// Install dependencies based on platform and components
    fn install_dependencies(
        &self, 
        core_only: bool, 
        with_python: bool, 
        component: Option<&str>,
        opsource: bool, 
        anya: bool, 
        anya_modules: Option<&str>,
        with_ml: bool, 
        ml_framework: Option<&str>
    ) -> Result<()> {
        println!("Installing dependencies...");
        
        if self.dry_run {
            println!("DRY RUN: Would install the following dependencies:");
            
            if core_only {
                println!("  - Would install core dependencies only");
            }
            
            if with_python {
                println!("  - Would install Python dependencies");
            }
            
            if let Some(comp) = component {
                println!("  - Would install {} component", comp);
            }
            
            if opsource {
                println!("  - Would install OPSource components");
            }
            
            if anya {
                println!("  - Would install anya-core components");
            }
            
            if let Some(modules) = anya_modules {
                println!("  - Would install anya-core modules: {}", modules);
            }
            
            if with_ml {
                println!("  - Would install machine learning dependencies");
            }
            
            if let Some(framework) = ml_framework {
                println!("  - Would install {} ML framework", framework);
            }
            
            return Ok(());
        }
        
        // Create installers for each component
        if opsource {
            let opsource_installer = OPSourceInstaller::new(
                self.project_root.clone(),
                self.config_manager.clone(),
                self.max_memory,
            )?;
            
            opsource_installer.install(core_only, component)?;
        }
        
        if anya {
            let anyacore_installer = AnyaCoreInstaller::new(
                self.project_root.clone(),
                self.config_manager.clone(),
                self.max_memory,
            )?;
            
            anyacore_installer.install(core_only, anya_modules)?;
        }
        
        // Install Python dependencies if requested
        if with_python {
            self.install_python_dependencies(core_only, component, opsource, anya, anya_modules)?;
        }
        
        // Install machine learning dependencies if requested
        if with_ml {
            let ml_installer = MLManager::new(
                &self.project_root,
                None,
                self.dry_run
            );
            
            ml_installer.install(ml_framework)?;
        }
        
        println!("✓ Dependencies installed successfully");
        Ok(())
    }
    
    /// Install Python dependencies
    fn install_python_dependencies(
        &self,
        core_only: bool,
        component: Option<&str>,
        opsource: bool,
        anya: bool,
        anya_modules: Option<&str>,
    ) -> Result<()> {
        println!("Installing Python dependencies...");
        
        if self.dry_run {
            println!("DRY RUN: Would install Python dependencies");
            return Ok(());
        }
        
        // Get the appropriate Python command
        let python_cmd = self.get_python_command()?;
        
        // Install pip if needed
        self.ensure_pip_installed(&python_cmd)?;
        
        // OPSource Python dependencies
        if opsource {
            println!("Installing OPSource Python dependencies...");
            
            let requirements_file = self.project_root.join("requirements.txt");
            if requirements_file.exists() {
                let status = Command::new(&python_cmd)
                    .args(&["-m", "pip", "install", "-r"])
                    .arg(&requirements_file)
                    .status()
                    .context("Failed to install Python dependencies")?;
                
                if !status.success() {
                    return Err(anyhow!("Failed to install Python dependencies"));
                }
            }
        }
        
        // Anya-core Python dependencies
        if anya {
            println!("Installing anya-core Python dependencies...");
            
            let anya_requirements = self.project_root.join("anya-core").join("requirements.txt");
            if anya_requirements.exists() {
                let status = Command::new(&python_cmd)
                    .args(&["-m", "pip", "install", "-r"])
                    .arg(&anya_requirements)
                    .status()
                    .context("Failed to install anya-core Python dependencies")?;
                
                if !status.success() {
                    return Err(anyhow!("Failed to install anya-core Python dependencies"));
                }
            }
        }
        
        println!("✓ Python dependencies installed successfully");
        Ok(())
    }
    
    /// Get the appropriate Python command based on the platform
    fn get_python_command(&self) -> Result<String> {
        // Try python3 first
        if Command::new("python3").arg("--version").output().is_ok() {
            return Ok("python3".to_string());
        }
        
        // Fall back to python
        if Command::new("python").arg("--version").output().is_ok() {
            return Ok("python".to_string());
        }
        
        Err(anyhow!("Python is not installed. Please install Python 3.x"))
    }
    
    /// Ensure pip is installed
    fn ensure_pip_installed(&self, python_cmd: &str) -> Result<()> {
        println!("Checking pip installation...");
        
        let pip_check = Command::new(python_cmd)
            .args(&["-m", "pip", "--version"])
            .output();
            
        if let Err(_) = pip_check {
            println!("pip not found, installing...");
            
            let get_pip_url = "https://bootstrap.pypa.io/get-pip.py";
            let get_pip_path = tempfile::tempdir()?.path().join("get-pip.py");
            
            // Download get-pip.py
            let response = reqwest::blocking::get(get_pip_url)
                .context("Failed to download get-pip.py")?;
                
            let content = response.text().context("Failed to read get-pip.py")?;
            fs::write(&get_pip_path, content).context("Failed to write get-pip.py")?;
            
            // Run get-pip.py
            let status = Command::new(python_cmd)
                .arg(&get_pip_path)
                .status()
                .context("Failed to run get-pip.py")?;
                
            if !status.success() {
                return Err(anyhow!("Failed to install pip"));
            }
        }
        
        println!("✓ pip is installed");
        Ok(())
    }
    
    // Configure application
    fn configure(
        &self,
        config_path: Option<&str>,
        set_values: &Vec<String>,
        show: bool,
    ) -> Result<()> {
        if self.dry_run {
            println!("DRY RUN: Would configure application");
            return Ok(());
        }
        
        self.config_manager.configure(config_path, set_values, show)?;
        Ok(())
    }
    
    // Run tests
    fn run_tests(&self, component: Option<&str>, json_output: bool) -> Result<()> {
        println!("Running tests...");
        
        if self.dry_run {
            println!("DRY RUN: Would run tests");
            return Ok(());
        }
        
        // Run general system tests
        self.system_detector.run_diagnostics()?;
        
        // Run OPSource tests
        let opsource_installer = OPSourceInstaller::new(
            self.project_root.clone(),
            self.config_manager.clone(),
            self.max_memory,
        )?;
        opsource_installer.test(component, json_output)?;
        
        // Run anya-core tests
        let anyacore_installer = AnyaCoreInstaller::new(
            self.project_root.clone(),
            self.config_manager.clone(),
            self.max_memory,
        )?;
        anyacore_installer.test(component, json_output)?;
        
        // Run ML tests if installed
        let ml_config = self.project_root.join("config").join("ml").join("ml_system_info.json");
        if ml_config.exists() {
            let ml_installer = MLManager::new(
                &self.project_root,
                None,
                self.dry_run
            );
            ml_installer.test(json_output)?;
        }
        
        // Run wallet and DAO tests if installed
        let wallet_config_path = self.project_root.join("data").join("wallet").join("wallet").join("wallet_config.json");
        if wallet_config_path.exists() || component.map_or(false, |c| c == "wallet" || c == "dao") {
            println!("Testing wallet and DAO components...");
            let wallet_manager = WalletManager::new(
                &self.project_root,
                self.dry_run
            );
            wallet_manager.test(json_output)?;
        }
        
        println!("✓ All tests completed successfully");
        Ok(())
    }
}

/// Find the project root directory
fn find_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    
    // Check if we're in the project root (has Cargo.toml)
    if current_dir.join("Cargo.toml").exists() {
        return Ok(current_dir);
    }
    
    // Try to find project root by going up directories
    let mut path = current_dir.clone();
    while path.parent().is_some() {
        path = path.parent().unwrap().to_path_buf();
        if path.join("Cargo.toml").exists() {
            return Ok(path);
        }
    }
    
    // If not found, use the current directory but issue a warning
    eprintln!("Warning: Could not find project root. Using current directory.");
    Ok(current_dir)
}

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create the installer with dry_run option
    let installer = Installer::new(cli.dry_run, match &cli.command {
        Commands::Install { max_memory, .. } => *max_memory,
        Commands::Test { max_memory, .. } => *max_memory,
        _ => None,
    })?;
    
    // Execute the appropriate command
    match &cli.command {
        Commands::Install { 
            core_only, 
            yes, 
            with_python, 
            component, 
            opsource, 
            anya, 
            anya_modules, 
            auto_config_anya, 
            with_ml, 
            ml_framework, 
            auto_config_ml,
            max_memory: _,  // Already processed in Installer::new
            setup_wallet,
            setup_dao
        } => {
            // Determine what to install if neither flag is specified
            let install_opsource = *opsource || (!*opsource && !*anya);
            let install_anya = *anya || (!*opsource && !*anya);
            
            // Print installation plan
            println!("Installation plan:");
            if install_opsource {
                println!("  - OPSource");
                if let Some(comp) = component {
                    println!("    Component: {}", comp);
                }
            }
            
            if install_anya {
                println!("  - anya-core");
                if let Some(modules) = anya_modules {
                    println!("    Modules: {}", modules);
                }
            }
            
            if *with_ml {
                println!("  - Machine Learning");
                if let Some(framework) = ml_framework {
                    println!("    Framework: {}", framework);
                }
            }
            
            if *setup_wallet {
                println!("  - Wallet Setup");
            }
            
            if *setup_dao {
                println!("  - DAO Setup");
            }
            
            // Ask for confirmation
            if !*yes && !cli.dry_run {
                print!("Do you want to continue? [y/N] ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Installation cancelled.");
                    return Ok(());
                }
            }
            
            // Check system requirements
            println!("Checking system requirements...");
            installer.system_detector.check_system_requirements()?;
            
            // Install dependencies
            installer.install_dependencies(
                *core_only, 
                *with_python, 
                component.as_deref(), 
                install_opsource, 
                install_anya, 
                anya_modules.as_deref(), 
                *with_ml, 
                ml_framework.as_deref()
            )?;
            
            // Configure with default settings
            installer.configure(None, &vec![], false)?;
            
            // Auto-configure anya-core if requested
            if *auto_config_anya && install_anya {
                println!("Auto-configuring anya-core...");
                let anyacore_installer = AnyaCoreInstaller::new(
                    installer.config_manager.clone(),
                    &installer.project_root,
                    installer.dry_run
                )?;
                
                // Extract anya modules
                let with_lightning = anya_modules
                    .as_ref()
                    .map(|s| s.contains("lightning"))
                    .unwrap_or(false);
                
                let with_web5 = anya_modules
                    .as_ref()
                    .map(|s| s.contains("web5"))
                    .unwrap_or(false);
                
                anyacore_installer.install(component.as_deref(), with_lightning, with_web5)?;
            }
            
            // Auto-configure ML if requested
            if *auto_config_ml && *with_ml {
                println!("Auto-configuring ML...");
                
                // Detect system information for ML configuration
                let system_info = installer.system_detector.detect_system_info()?;
                
                let ml_manager = MLManager::new(
                    &installer.project_root,
                    Some(system_info),
                    installer.dry_run
                );
                
                // Parse frameworks
                let frameworks = if let Some(framework) = ml_framework {
                    match framework.as_str() {
                        "tensorflow" => vec!["tensorflow".to_string()],
                        "pytorch" => vec!["pytorch".to_string()],
                        "both" => vec!["tensorflow".to_string(), "pytorch".to_string()],
                        _ => vec!["tensorflow".to_string()]  // Default to TensorFlow
                    }
                } else {
                    vec!["tensorflow".to_string()]  // Default to TensorFlow
                };
                
                ml_manager.install(&frameworks, true)?;
            }
            
            // Setup wallet and DAO if requested
            if *setup_wallet || *setup_dao {
                println!("Setting up wallet and DAO components...");
                
                let wallet_manager = WalletManager::new(
                    &installer.project_root,
                    installer.dry_run
                );
                
                // Get Bitcoin network from config
                let config = installer.config_manager.get_config();
                let bitcoin_network = &config.bitcoin_network;
                
                wallet_manager.setup(*setup_wallet, *setup_dao, bitcoin_network)?;
                
                // Configure Taproot and DLC if enabled
                if config.enable_taproot {
                    wallet_manager.configure_taproot(true)?;
                }
                
                if config.enable_dlc {
                    wallet_manager.configure_dlc(true)?;
                }
                
                // Setup DAO metrics if DAO is enabled
                if *setup_dao {
                    wallet_manager.setup_dao_metrics()?;
                }
            }
            
            println!("\nInstallation completed successfully!");
            println!("To configure OPSource, run: cargo run --bin installer -- configure");
            println!("To test the installation, run: cargo run --bin installer -- test");
        },
        
        Commands::Test { component, json, max_memory: _ } => {
            installer.run_tests(component.as_deref(), *json)?;
        },
        
        Commands::Configure { config, set, show } => {
            installer.configure(config.as_deref(), set, *show)?;
        },
    }
    
    Ok(())
}
