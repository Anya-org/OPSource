use std::path::PathBuf;
use std::process::Command;
use anyhow::{Result, anyhow, Context};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// Check if a command is available on the system
pub fn is_command_available(command: &str) -> bool {
    match std::env::consts::OS {
        "windows" => {
            Command::new("where")
                .arg(command)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        },
        _ => {
            Command::new("which")
                .arg(command)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }
}

/// Get the path to a command
pub fn get_command_path(command: &str) -> Result<PathBuf> {
    match std::env::consts::OS {
        "windows" => {
            let output = Command::new("where")
                .arg(command)
                .output()
                .context(format!("Failed to locate command: {}", command))?;
                
            if !output.status.success() {
                return Err(anyhow!("Command not found: {}", command));
            }
            
            let path_str = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .ok_or_else(|| anyhow!("Empty output from 'where {}'", command))?
                .trim()
                .to_string();
                
            Ok(PathBuf::from(path_str))
        },
        _ => {
            let output = Command::new("which")
                .arg(command)
                .output()
                .context(format!("Failed to locate command: {}", command))?;
                
            if !output.status.success() {
                return Err(anyhow!("Command not found: {}", command));
            }
            
            let path_str = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string();
                
            Ok(PathBuf::from(path_str))
        }
    }
}

/// Generate a random password
pub fn generate_random_password(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Determine if a component should be installed based on component specifier
pub fn should_install_component(component: Option<&str>, target_component: &str) -> bool {
    match component {
        Some("all") => true,
        Some(comp) if comp == target_component => true,
        Some(_) => false,
        None => true,
    }
}

/// Convert bytes to a human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Test if a port is available for use
pub fn is_port_available(port: u16) -> bool {
    use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr};
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    TcpListener::bind(addr).is_ok()
}

/// Find an available port starting from a base port
pub fn find_available_port(base_port: u16) -> Option<u16> {
    (base_port..65535).find(|&port| is_port_available(port))
}

/// Parse memory limit from string (supports KB, MB, GB suffixes)
pub fn parse_memory_limit(limit: &str) -> Result<u64> {
    let limit = limit.trim().to_lowercase();
    
    if limit.ends_with("kb") {
        let num = limit.trim_end_matches("kb").trim().parse::<u64>()?;
        Ok(num * 1024)
    } else if limit.ends_with("mb") {
        let num = limit.trim_end_matches("mb").trim().parse::<u64>()?;
        Ok(num * 1024 * 1024)
    } else if limit.ends_with("gb") {
        let num = limit.trim_end_matches("gb").trim().parse::<u64>()?;
        Ok(num * 1024 * 1024 * 1024)
    } else {
        // Assume bytes
        Ok(limit.parse::<u64>()?)
    }
}
