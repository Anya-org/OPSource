#!/usr/bin/env python3
"""
Test script for OPSource and anya-core installers.
This script will run both installers in dry-run mode to check all modules.
"""

import os
import sys
import subprocess
import importlib
import platform
import json
from pathlib import Path
from typing import Dict, List, Tuple, Set

# Define colors for terminal output
class Colors:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def print_header(message: str) -> None:
    """Print a formatted header message."""
    print(f"\n{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{message.center(80)}{Colors.ENDC}")
    print(f"{Colors.HEADER}{Colors.BOLD}{'=' * 80}{Colors.ENDC}\n")

def print_status(message: str, status: bool) -> None:
    """Print a formatted status message."""
    status_text = f"{Colors.GREEN}[PASS]{Colors.ENDC}" if status else f"{Colors.RED}[FAIL]{Colors.ENDC}"
    print(f"{status_text} {message}")

def get_installed_packages() -> Set[str]:
    """Get a set of all installed Python packages."""
    try:
        result = subprocess.run(
            [sys.executable, "-m", "pip", "freeze"],
            capture_output=True,
            text=True,
            check=True
        )
        packages = set()
        for line in result.stdout.splitlines():
            if "==" in line:
                package_name = line.split("==")[0].lower()
                packages.add(package_name)
        return packages
    except subprocess.CalledProcessError:
        print(f"{Colors.RED}Failed to get installed packages{Colors.ENDC}")
        return set()

def check_dependencies(requirements_file: Path) -> Tuple[List[str], List[str]]:
    """Check if all dependencies from a requirements file are installed."""
    if not requirements_file.exists():
        return [], [f"Requirements file not found: {requirements_file}"]
    
    missing_packages = []
    installed_packages = []
    installed = get_installed_packages()
    
    try:
        with open(requirements_file, 'r') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    # Extract package name (remove version and options)
                    package_name = line.split('==')[0].split('>=')[0].split('<=')[0].strip().lower()
                    if package_name:
                        if package_name in installed:
                            installed_packages.append(package_name)
                        else:
                            missing_packages.append(package_name)
    except Exception as e:
        missing_packages.append(f"Error reading requirements file: {str(e)}")
    
    return installed_packages, missing_packages

def check_cargo_dependencies(cargo_toml: Path) -> Tuple[List[str], List[str]]:
    """Check if Rust dependencies from Cargo.toml are available."""
    if not cargo_toml.exists():
        return [], [f"Cargo.toml file not found: {cargo_toml}"]
    
    available_deps = []
    missing_deps = []
    
    try:
        # Check if cargo is installed
        result = subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            return [], ["Cargo is not installed or not in PATH"]
            
        # For Rust, we just check if cargo can find the dependencies
        # This doesn't actually build them, just checks availability
        result = subprocess.run(
            ["cargo", "fetch", "--manifest-path", str(cargo_toml)],
            capture_output=True,
            text=True
        )
        
        if result.returncode == 0:
            # Successful fetch means all dependencies are available
            # But we don't have detailed info without parsing Cargo.toml
            available_deps.append("All Cargo dependencies available")
        else:
            missing_deps.append("Some Cargo dependencies are not available")
            # Add error details
            for line in result.stderr.splitlines():
                if "error:" in line:
                    missing_deps.append(f"  {line.strip()}")
    except Exception as e:
        missing_deps.append(f"Error checking Cargo dependencies: {str(e)}")
    
    return available_deps, missing_deps

def run_installer_dry_run(script_path: Path) -> Tuple[bool, str]:
    """Run an installer in dry-run mode."""
    if not script_path.exists():
        return False, f"Installer script not found: {script_path}"
    
    try:
        if script_path.name.endswith(".py"):
            # Python installer
            result = subprocess.run(
                [sys.executable, str(script_path), "--dry-run"],
                capture_output=True,
                text=True
            )
        elif script_path.name.endswith(".rs") or script_path.name == "installer":
            # Rust installer - we need to build it first if it's a .rs file
            if script_path.name.endswith(".rs"):
                build_result = subprocess.run(
                    ["cargo", "build", "--bin", "installer"],
                    cwd=script_path.parent.parent.parent,  # Navigate to project root
                    capture_output=True,
                    text=True
                )
                if build_result.returncode != 0:
                    return False, f"Failed to build installer: {build_result.stderr}"
                
                # Use the built executable
                installer_path = script_path.parent.parent.parent / "target" / "debug" / "installer"
            else:
                installer_path = script_path
                
            result = subprocess.run(
                [str(installer_path), "install", "--dry-run", "--yes"],
                capture_output=True,
                text=True
            )
        else:
            return False, f"Unsupported installer format: {script_path}"
            
        if result.returncode == 0:
            return True, result.stdout
        else:
            return False, f"Installer failed with exit code {result.returncode}:\n{result.stderr}"
    except Exception as e:
        return False, f"Error running installer: {str(e)}"

def test_system_compatibility() -> Dict[str, bool]:
    """Test system compatibility for installation."""
    results = {}
    
    # Check OS compatibility
    if platform.system() == "Windows":
        results["Windows OS"] = True
    elif platform.system() == "Linux":
        results["Linux OS"] = True
    elif platform.system() == "Darwin":
        results["macOS"] = True
    else:
        results["Unsupported OS"] = False
    
    # Check Python version
    python_version = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"
    results[f"Python {python_version}"] = sys.version_info.major == 3 and sys.version_info.minor >= 8
    
    # Check for Rust
    try:
        result = subprocess.run(
            ["rustc", "--version"],
            capture_output=True,
            text=True
        )
        results["Rust"] = result.returncode == 0
    except:
        results["Rust"] = False
    
    # Check for cargo
    try:
        result = subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            text=True
        )
        results["Cargo"] = result.returncode == 0
    except:
        results["Cargo"] = False
    
    return results

def test_opsource_installer(base_dir: Path) -> None:
    """Test the OPSource installer."""
    print_header("TESTING OPSOURCE INSTALLER")
    
    # Define paths
    setup_dev_py = base_dir / "setup_dev.py"
    requirements_txt = base_dir / "requirements.txt"
    cargo_toml = base_dir / "Cargo.toml"
    
    # Check if installer exists
    if not setup_dev_py.exists():
        print(f"{Colors.RED}OPSource installer not found at {setup_dev_py}{Colors.ENDC}")
        return
    
    # Check Python dependencies
    print(f"{Colors.BLUE}{Colors.BOLD}Checking Python dependencies...{Colors.ENDC}")
    installed, missing = check_dependencies(requirements_txt)
    
    for pkg in installed:
        print_status(f"Package {pkg}", True)
    
    for pkg in missing:
        print_status(f"Package {pkg}", False)
    
    # Check Cargo dependencies
    if cargo_toml.exists():
        print(f"\n{Colors.BLUE}{Colors.BOLD}Checking Cargo dependencies...{Colors.ENDC}")
        available, unavailable = check_cargo_dependencies(cargo_toml)
        
        for dep in available:
            print_status(dep, True)
        
        for dep in unavailable:
            print_status(dep, False)
    
    # Run installer in dry run mode
    print(f"\n{Colors.BLUE}{Colors.BOLD}Running installer in dry-run mode...{Colors.ENDC}")
    success, output = run_installer_dry_run(setup_dev_py)
    print_status("Installer dry run", success)
    
    if not success:
        print(f"{Colors.RED}Installer output:{Colors.ENDC}")
        print(output)
    else:
        print(f"{Colors.GREEN}Installer output:{Colors.ENDC}")
        print(output)

def test_anya_core_installer(base_dir: Path) -> None:
    """Test the anya-core installer."""
    print_header("TESTING ANYA-CORE INSTALLER")
    
    # Define paths
    installer_py = base_dir / "scripts" / "install.py"
    installer_rs = base_dir / "src" / "bin" / "installer.rs"
    requirements_txt = base_dir / "requirements.txt"
    cargo_toml = base_dir / "Cargo.toml"
    
    # Check Python dependencies if installer.py exists
    if installer_py.exists():
        print(f"{Colors.BLUE}{Colors.BOLD}Checking Python dependencies...{Colors.ENDC}")
        installed, missing = check_dependencies(requirements_txt)
        
        for pkg in installed:
            print_status(f"Package {pkg}", True)
        
        for pkg in missing:
            print_status(f"Package {pkg}", False)
        
        # Run Python installer in dry run mode
        print(f"\n{Colors.BLUE}{Colors.BOLD}Running Python installer in dry-run mode...{Colors.ENDC}")
        success, output = run_installer_dry_run(installer_py)
        print_status("Python installer dry run", success)
        
        if not success:
            print(f"{Colors.RED}Installer output:{Colors.ENDC}")
            print(output)
        else:
            print(f"{Colors.GREEN}Installer output:{Colors.ENDC}")
            print(output)
    
    # Check Rust dependencies and installer
    if installer_rs.exists() and cargo_toml.exists():
        print(f"\n{Colors.BLUE}{Colors.BOLD}Checking Cargo dependencies...{Colors.ENDC}")
        available, unavailable = check_cargo_dependencies(cargo_toml)
        
        for dep in available:
            print_status(dep, True)
        
        for dep in unavailable:
            print_status(dep, False)
        
        # Run Rust installer in dry run mode
        print(f"\n{Colors.BLUE}{Colors.BOLD}Running Rust installer in dry-run mode...{Colors.ENDC}")
        success, output = run_installer_dry_run(installer_rs)
        print_status("Rust installer dry run", success)
        
        if not success:
            print(f"{Colors.RED}Installer output:{Colors.ENDC}")
            print(output)
        else:
            print(f"{Colors.GREEN}Installer output:{Colors.ENDC}")
            print(output)

def main() -> None:
    """Main entry point."""
    base_dir = Path(__file__).resolve().parent
    
    print_header("SYSTEM COMPATIBILITY CHECK")
    
    # Check system compatibility
    compatibility = test_system_compatibility()
    for component, status in compatibility.items():
        print_status(component, status)
    
    # Test OPSource installer
    test_opsource_installer(base_dir)
    
    # Test anya-core installer
    test_anya_core_installer(base_dir / "anya-core")
    
    print_header("TEST SUMMARY")
    
    print(f"""
{Colors.BOLD}Test completed. Check the output above for any issues that need to be addressed.{Colors.ENDC}

{Colors.YELLOW}Recommended next steps:{Colors.ENDC}
1. Fix any missing dependencies highlighted in red.
2. Address any installer errors or warnings.
3. For a full installation, run the installers without the --dry-run flag.
""")

if __name__ == "__main__":
    main()
