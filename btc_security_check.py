#!/usr/bin/env python3
"""
Bitcoin-Specific Security Verification Tool
------------------------------------------
This script performs security checks specific to Bitcoin applications,
focusing on cryptographic implementations, key management, and 
transaction signing security.
"""

import os
import re
import sys
import hashlib
import subprocess
from pathlib import Path
from typing import List, Dict, Tuple, Any, Optional

# Define color codes for output
class Colors:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def print_header(message: str) -> None:
    """Print a formatted header message."""
    print(f"\n{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{message.center(80)}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}\n")

def print_section(message: str) -> None:
    """Print a formatted section message."""
    print(f"\n{Colors.BLUE}{Colors.BOLD}{message}{Colors.ENDC}")
    print(f"{Colors.BLUE}{'-' * len(message)}{Colors.ENDC}")

def print_warning(message: str) -> None:
    """Print a formatted warning message."""
    print(f"{Colors.WARNING}WARNING: {message}{Colors.ENDC}")

def print_error(message: str) -> None:
    """Print a formatted error message."""
    print(f"{Colors.FAIL}ERROR: {message}{Colors.ENDC}")

def print_success(message: str) -> None:
    """Print a formatted success message."""
    print(f"{Colors.GREEN}SUCCESS: {message}{Colors.ENDC}")

def check_bitcoin_dependencies() -> List[Dict[str, str]]:
    """Check for installed Bitcoin-related dependencies and their versions."""
    print_section("Checking Bitcoin Dependencies")
    
    bitcoin_packages = [
        "bitcoin",
        "bitcoincore-rpc",
        "python-bitcoinlib",
        "python-bitcoinrpc",
        "hdwallet",
        "mnemonic",
        "coincurve",
        "ecdsa",
        "cryptography",
        "pycryptodome"
    ]
    
    results = []
    
    for package in bitcoin_packages:
        try:
            # Use pip to get the installed version
            result = subprocess.run(
                [sys.executable, "-m", "pip", "show", package],
                capture_output=True,
                text=True,
                check=False
            )
            
            if result.returncode == 0:
                # Extract the version from pip output
                version_match = re.search(r"Version: (.+)", result.stdout)
                version = version_match.group(1) if version_match else "Unknown"
                
                # Check for known vulnerable versions
                status = "OK"
                if package == "cryptography" and version < "44.0.2":
                    status = "VULNERABLE - Update to 44.0.2 or later"
                elif package == "pycryptodome" and version < "3.21.1":
                    status = "VULNERABLE - Update to 3.21.1 or later"
                
                results.append({
                    "package": package,
                    "version": version,
                    "status": status
                })
                
                print(f"  {package}: {version} - {status}")
            else:
                print(f"  {package}: Not installed")
        except Exception as e:
            print(f"  {package}: Error checking - {str(e)}")
    
    return results

def check_rust_security() -> List[Dict[str, Any]]:
    """Check Rust codebase for security best practices."""
    print_section("Checking Rust Security Best Practices")
    
    results = []
    
    # Check if cargo audit is installed
    try:
        result = subprocess.run(
            ["cargo", "audit", "--version"],
            capture_output=True,
            text=True,
            check=False
        )
        
        if result.returncode == 0:
            print("  cargo-audit is installed, checking dependencies...")
            
            # Run cargo audit
            audit_result = subprocess.run(
                ["cargo", "audit"],
                capture_output=True,
                text=True,
                check=False
            )
            
            if "No vulnerable packages found" in audit_result.stdout:
                print_success("No vulnerable Rust dependencies found")
            else:
                print_warning("Vulnerable Rust dependencies found:")
                print(audit_result.stdout)
                
                # Extract vulnerable packages
                vuln_matches = re.finditer(r"([a-zA-Z0-9_-]+) (\d+\.\d+\.\d+)", audit_result.stdout)
                for match in vuln_matches:
                    results.append({
                        "package": match.group(1),
                        "version": match.group(2),
                        "status": "VULNERABLE"
                    })
        else:
            print_warning("cargo-audit not installed. Run 'cargo install cargo-audit' to enable Rust security checks.")
    except Exception as e:
        print_error(f"Error checking Rust security: {str(e)}")
    
    return results

def check_bitcoin_specific_patterns() -> None:
    """Check for Bitcoin-specific security patterns."""
    print_section("Checking Bitcoin-Specific Security Patterns")
    
    bitcoin_patterns = {
        r"private_?key": "Possible private key handling",
        r"mnemonic": "Possible mnemonic phrase handling",
        r"wallet": "Wallet implementation found",
        r"(?:seed|hd)_phrase": "HD wallet seed phrase handling",
        r"bitcoin_?rpc": "Bitcoin RPC connection",
        r"(?:testnet|mainnet)": "Network selection code",
        r"(?:taproot|schnorr|segwit)": "Advanced Bitcoin cryptography usage",
    }
    
    for root, _, files in os.walk("."):
        # Skip virtual environment directories and test directories
        if any(skip in root for skip in ["venv", ".venv", "env", "node_modules", ".git", "test"]):
            continue
            
        for file_name in files:
            if file_name.endswith((".py", ".rs", ".js", ".ts")):
                file_path = os.path.join(root, file_name)
                
                try:
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        
                        for pattern, description in bitcoin_patterns.items():
                            matches = re.finditer(pattern, content, re.IGNORECASE)
                            for match in matches:
                                line_no = content[:match.start()].count('\n') + 1
                                context_start = max(0, line_no - 2)
                                context_end = min(content.count('\n'), line_no + 2)
                                context_lines = content.split('\n')[context_start:context_end]
                                
                                print(f"  {Colors.CYAN}Found {description} in {file_path}:{line_no}{Colors.ENDC}")
                                for i, line in enumerate(context_lines, start=context_start + 1):
                                    prefix = "→ " if i == line_no else "  "
                                    print(f"    {prefix}{line.strip()}")
                                print()
                except Exception as e:
                    print(f"  Error reading {file_path}: {str(e)}")

def check_rust_migration_readiness() -> None:
    """Check if the codebase is ready for Rust migration."""
    print_section("Checking Rust Migration Readiness")
    
    # Look for Python code that might be difficult to migrate
    complex_patterns = {
        r"multiprocessing": "Python multiprocessing (complex to migrate)",
        r"(?:numpy|pandas|torch|tensorflow)": "ML/Data Science libraries (complex to migrate)",
        r"ctypes": "C bindings (may need FFI in Rust)",
        r"asyncio": "Async Python code (different paradigm in Rust)",
        r"decorator": "Python decorators (different paradigm in Rust)"
    }
    
    # Check for existing Rust code
    rust_files = []
    for root, _, files in os.walk("."):
        for file in files:
            if file.endswith(".rs"):
                rust_files.append(os.path.join(root, file))
    
    if rust_files:
        print_success(f"Found {len(rust_files)} Rust files already in the codebase.")
        for file in rust_files[:5]:  # Show first 5 files
            print(f"  - {file}")
        if len(rust_files) > 5:
            print(f"  ... and {len(rust_files) - 5} more")
    else:
        print_warning("No Rust files found in the codebase yet.")
    
    # Check for Cargo.toml
    if os.path.exists("Cargo.toml"):
        print_success("Cargo.toml file found, indicating Rust project setup.")
    else:
        print_warning("No Cargo.toml file found. You'll need to set up a Rust project.")
    
    # Scan for Python code that might be challenging to migrate
    for root, _, files in os.walk("."):
        if any(skip in root for skip in ["venv", ".venv", "env", "node_modules", ".git", "test"]):
            continue
            
        for file_name in files:
            if file_name.endswith(".py"):
                file_path = os.path.join(root, file_name)
                
                try:
                    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                        
                        for pattern, description in complex_patterns.items():
                            if re.search(pattern, content, re.IGNORECASE):
                                print_warning(f"Found {description} in {file_path}")
                                break
                except Exception as e:
                    print(f"  Error reading {file_path}: {str(e)}")
    
def main() -> None:
    """Main function to run all checks."""
    print_header("Bitcoin Security and Rust Migration Readiness Check")
    
    check_bitcoin_dependencies()
    check_rust_security()
    check_bitcoin_specific_patterns()
    check_rust_migration_readiness()
    
    print_header("Check Complete")
    print("Review the results above and address any security concerns or migration challenges.")
    print("For security vulnerabilities, update to the recommended versions.")
    print("For Rust migration, start with simpler components that have fewer challenging patterns.")

if __name__ == "__main__":
    main()
