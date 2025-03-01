use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;

use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};

use crate::utils::generate_random_password;

/// Configuration manager for the installer
pub struct ConfigManager {
    project_root: PathBuf,
    config_dir: PathBuf,
    data_dir: PathBuf,
    config: InstallConfig,
}

/// Main installation configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InstallConfig {
    pub bitcoin_network: String,
    pub log_level: String,
    pub data_dir: String,
    pub bitcoin_rpc_user: String,
    pub bitcoin_rpc_password: String,
    pub bitcoin_rpc_port: u16,
    pub enable_taproot: bool,
    pub enable_rgb: bool,
    pub enable_dlc: bool,
    pub enable_rsk: bool,
    pub enable_web5: bool,
    pub enable_stacks: bool,
    pub components: HashMap<String, bool>,
    pub max_memory_mb: Option<u64>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        let mut components = HashMap::new();
        components.insert("bitcoin".to_string(), true);
        components.insert("web5".to_string(), true);
        components.insert("rgb".to_string(), true);
        components.insert("dlc".to_string(), true);
        components.insert("taproot".to_string(), true);
        components.insert("rsk".to_string(), true);
        components.insert("stacks".to_string(), true);
        
        Self {
            bitcoin_network: "testnet".to_string(),
            log_level: "info".to_string(),
            data_dir: "./data".to_string(),
            bitcoin_rpc_user: "opsource".to_string(),
            bitcoin_rpc_password: generate_random_password(16),
            bitcoin_rpc_port: 18332,
            enable_taproot: true,
            enable_rgb: true,
            enable_dlc: true,
            enable_rsk: true,
            enable_web5: true,
            enable_stacks: true,
            components,
            max_memory_mb: None,
        }
    }
}

/// Configuration for Bitcoin node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub network: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub rpc_port: u16,
    pub rpc_allow_ip: String,
    pub server: bool,
    pub data_dir: String,
    pub prune: Option<u32>,
    pub txindex: bool,
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            rpc_user: "opsource".to_string(),
            rpc_password: generate_random_password(16),
            rpc_port: 18332,
            rpc_allow_ip: "127.0.0.1".to_string(),
            server: true,
            data_dir: "./data/bitcoin".to_string(),
            prune: None,
            txindex: true,
        }
    }
}

/// Configuration for anya-core components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaConfig {
    /// Bitcoin node configuration
    pub bitcoin: AnyaBitcoinConfig,
    
    /// Lightning configuration
    pub lightning: Option<AnyaLightningConfig>,
    
    /// Web5 configuration
    pub web5: Option<AnyaWeb5Config>,
    
    /// Extensions configuration
    pub extensions: Option<AnyaExtensionsConfig>,
}

/// Bitcoin configuration for anya-core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaBitcoinConfig {
    /// Bitcoin network (mainnet, testnet, regtest)
    pub network: String,
    
    /// RPC user
    pub rpc_user: String,
    
    /// RPC password
    pub rpc_password: String,
    
    /// RPC port
    pub rpc_port: u16,
    
    /// Data directory
    pub data_dir: String,
}

/// Lightning configuration for anya-core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaLightningConfig {
    /// Enable Lightning
    pub enabled: bool,
    
    /// Lightning implementation (LDK, CLN, LND)
    pub implementation: String,
    
    /// Lightning network (mainnet, testnet, regtest)
    pub network: String,
    
    /// Lightning data directory
    pub data_dir: String,
}

/// Web5 configuration for anya-core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaWeb5Config {
    /// Enable Web5
    pub enabled: bool,
    
    /// DID method (key, web, ion)
    pub did_method: String,
    
    /// Web5 server port
    pub port: u16,
}

/// Extensions configuration for anya-core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyaExtensionsConfig {
    /// Enable extensions
    pub enabled: bool,
    
    /// Extensions directory
    pub extensions_dir: String,
    
    /// Extensions to enable
    pub enabled_extensions: Vec<String>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new(project_root: &Path) -> Result<Self> {
        let config_dir = project_root.join("config");
        let data_dir = project_root.join("data");
        
        // Ensure directories exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context(format!("Failed to create config directory: {:?}", config_dir))?;
        }
        
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .context(format!("Failed to create data directory: {:?}", data_dir))?;
        }
        
        // Load or create default config
        let config_path = config_dir.join("config.json");
        let config = if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)
                .context(format!("Failed to read config file: {:?}", config_path))?;
                
            serde_json::from_str(&config_str)
                .context("Failed to parse config file")?
        } else {
            let default_config = InstallConfig::default();
            
            let config_json = serde_json::to_string_pretty(&default_config)
                .context("Failed to serialize default config")?;
                
            fs::write(&config_path, config_json)
                .context(format!("Failed to write default config to {:?}", config_path))?;
                
            default_config
        };
        
        Ok(Self {
            project_root: project_root.to_path_buf(),
            config_dir,
            data_dir,
            config,
        })
    }
    
    /// Get the current configuration
    pub fn get_config(&self) -> &InstallConfig {
        &self.config
    }
    
    /// Update the configuration
    pub fn update_config(&mut self, new_config: InstallConfig) -> Result<()> {
        self.config = new_config;
        self.save_config()
    }
    
    /// Save the current configuration
    pub fn save_config(&self) -> Result<()> {
        let config_path = self.config_dir.join("config.json");
        
        let config_json = serde_json::to_string_pretty(&self.config)
            .context("Failed to serialize config")?;
            
        fs::write(&config_path, config_json)
            .context(format!("Failed to write config to {:?}", config_path))?;
            
        Ok(())
    }
    
    /// Configure the application
    pub fn configure(&self, config_path: Option<&str>, set_values: &Vec<String>, show: bool) -> Result<()> {
        println!("Configuring application...");
        
        if show {
            // Show current configuration
            let config_json = serde_json::to_string_pretty(&self.config)
                .context("Failed to serialize config")?;
                
            println!("Current configuration:");
            println!("{}", config_json);
            
            return Ok(());
        }
        
        if let Some(path) = config_path {
            // Load configuration from file
            let custom_config_path = PathBuf::from(path);
            
            if !custom_config_path.exists() {
                return Err(anyhow!("Configuration file not found: {:?}", custom_config_path));
            }
            
            let config_str = fs::read_to_string(&custom_config_path)
                .context(format!("Failed to read config file: {:?}", custom_config_path))?;
                
            let new_config: InstallConfig = serde_json::from_str(&config_str)
                .context("Failed to parse config file")?;
                
            // Update the configuration
            let mut config_manager = self.clone();
            config_manager.update_config(new_config)?;
            
            println!("✓ Configuration loaded from {:?}", custom_config_path);
            
            return Ok(());
        }
        
        if !set_values.is_empty() {
            // Update specific configuration values
            let mut new_config = self.config.clone();
            
            for set_value in set_values {
                let parts: Vec<&str> = set_value.splitn(2, '=').collect();
                
                if parts.len() != 2 {
                    return Err(anyhow!("Invalid set value format: {}. Expected key=value", set_value));
                }
                
                let key = parts[0].trim();
                let value = parts[1].trim();
                
                match key {
                    "bitcoin_network" => new_config.bitcoin_network = value.to_string(),
                    "log_level" => new_config.log_level = value.to_string(),
                    "data_dir" => new_config.data_dir = value.to_string(),
                    "bitcoin_rpc_user" => new_config.bitcoin_rpc_user = value.to_string(),
                    "bitcoin_rpc_password" => new_config.bitcoin_rpc_password = value.to_string(),
                    "bitcoin_rpc_port" => {
                        new_config.bitcoin_rpc_port = value.parse::<u16>()
                            .context(format!("Invalid port number: {}", value))?;
                    },
                    "enable_taproot" => {
                        new_config.enable_taproot = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "enable_rgb" => {
                        new_config.enable_rgb = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "enable_dlc" => {
                        new_config.enable_dlc = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "enable_rsk" => {
                        new_config.enable_rsk = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "enable_web5" => {
                        new_config.enable_web5 = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "enable_stacks" => {
                        new_config.enable_stacks = value.parse::<bool>()
                            .context(format!("Invalid boolean value: {}", value))?;
                    },
                    "max_memory_mb" => {
                        if value.to_lowercase() == "none" {
                            new_config.max_memory_mb = None;
                        } else {
                            new_config.max_memory_mb = Some(value.parse::<u64>()
                                .context(format!("Invalid memory value: {}", value))?);
                        }
                    },
                    _ => {
                        if key.starts_with("component.") {
                            let component_name = key.trim_start_matches("component.");
                            let enable = value.parse::<bool>()
                                .context(format!("Invalid boolean value: {}", value))?;
                            
                            new_config.components.insert(component_name.to_string(), enable);
                        } else {
                            return Err(anyhow!("Unknown configuration key: {}", key));
                        }
                    }
                }
                
                println!("✓ Set {}={}", key, value);
            }
            
            // Update the configuration
            let mut config_manager = self.clone();
            config_manager.update_config(new_config)?;
            
            self.generate_bitcoin_conf()?;
        }
        
        println!("✓ Configuration updated");
        Ok(())
    }
    
    /// Generate bitcoin.conf file
    pub fn generate_bitcoin_conf(&self) -> Result<()> {
        println!("Generating Bitcoin configuration...");
        
        let bitcoin_dir = PathBuf::from(&self.config.data_dir).join("bitcoin");
        
        // Ensure Bitcoin directory exists
        if !bitcoin_dir.exists() {
            fs::create_dir_all(&bitcoin_dir)
                .context(format!("Failed to create Bitcoin directory: {:?}", bitcoin_dir))?;
        }
        
        let conf_path = bitcoin_dir.join("bitcoin.conf");
        
        // Generate bitcoin.conf content
        let mut conf_content = String::new();
        
        // Add network-specific settings
        match self.config.bitcoin_network.as_str() {
            "mainnet" => {
                conf_content.push_str("# Bitcoin mainnet configuration\n");
                conf_content.push_str("chain=main\n");
            },
            "testnet" => {
                conf_content.push_str("# Bitcoin testnet configuration\n");
                conf_content.push_str("chain=test\n");
                conf_content.push_str("testnet=1\n");
            },
            "regtest" => {
                conf_content.push_str("# Bitcoin regtest configuration\n");
                conf_content.push_str("chain=regtest\n");
                conf_content.push_str("regtest=1\n");
            },
            _ => return Err(anyhow!("Unknown Bitcoin network: {}", self.config.bitcoin_network)),
        }
        
        // Add RPC configuration
        conf_content.push_str("\n# RPC configuration\n");
        conf_content.push_str(&format!("rpcuser={}\n", self.config.bitcoin_rpc_user));
        conf_content.push_str(&format!("rpcpassword={}\n", self.config.bitcoin_rpc_password));
        conf_content.push_str(&format!("rpcport={}\n", self.config.bitcoin_rpc_port));
        conf_content.push_str("rpcallowip=127.0.0.1\n");
        conf_content.push_str("server=1\n");
        
        // Add Taproot settings if enabled
        if self.config.enable_taproot {
            conf_content.push_str("\n# Taproot settings\n");
            conf_content.push_str("txindex=1\n");
        }
        
        // Add memory limit if configured
        if let Some(max_mem) = self.config.max_memory_mb {
            conf_content.push_str("\n# Memory configuration\n");
            conf_content.push_str(&format!("dbcache={}\n", max_mem.min(4096) / 2)); // Use half of max memory, but not more than 2GB
            conf_content.push_str(&format!("maxmempool={}\n", max_mem.min(1024))); // At most 1GB for mempool
        }
        
        // Write the configuration file
        fs::write(&conf_path, conf_content)
            .context(format!("Failed to write Bitcoin configuration to {:?}", conf_path))?;
            
        println!("✓ Bitcoin configuration saved to {:?}", conf_path);
        Ok(())
    }
    
    /// Clone this ConfigManager
    pub fn clone(&self) -> Self {
        Self {
            project_root: self.project_root.clone(),
            config_dir: self.config_dir.clone(),
            data_dir: self.data_dir.clone(),
            config: self.config.clone(),
        }
    }
}
