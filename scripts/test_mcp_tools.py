#!/usr/bin/env python3
"""
MCP Tools Test Script
This script tests the MCP tools to ensure they are functioning properly.
"""

import subprocess
import json
import os
import sys

def print_header(text):
    """Print a formatted header"""
    print("\n" + "="*80)
    print(f" {text} ".center(80, "="))
    print("="*80 + "\n")

def run_command(command):
    """Run a command and return the output"""
    try:
        result = subprocess.run(
            command,
            capture_output=True,
            text=True,
            check=True
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running command: {e}")
        print(f"stderr: {e.stderr}")
        return None

def test_bitcoin_protocol_validator():
    """Test the Bitcoin Protocol Validator tool"""
    print_header("Testing Bitcoin Protocol Validator")
    
    test_cases = [
        "Verify this transaction follows BIP-341 taproot standard: tr(KEY,{SILENT_LEAF})",
        "Check PSBT format: psbt:0123456789abcdef",
        "Invalid input with no protocol references"
    ]
    
    for i, test_case in enumerate(test_cases):
        print(f"Test case {i+1}: {test_case}")
        output = run_command(["python", "scripts/validate_bitcoin_protocol.py", "--input", test_case])
        
        if output:
            try:
                result = json.loads(output)
                print(f"Result: {'Compliant' if result.get('compliant', False) else 'Non-compliant'}")
                print(f"Standards checked: {', '.join(result.get('standards_checked', []))}")
                if 'warning' in result:
                    print(f"Warning: {result['warning']}")
                print(f"Details: {json.dumps(result.get('details', []), indent=2)}")
            except json.JSONDecodeError:
                print("Error: Invalid JSON response")
        
        print("-" * 40)

def test_taproot_asset_creator():
    """Test the Taproot Asset Creator tool"""
    print_header("Testing Taproot Asset Creator")
    
    test_cases = [
        "Create asset named TestToken with supply 1000000",
        "Create asset named PrivacyCoin with supply 21000000"
    ]
    
    for i, test_case in enumerate(test_cases):
        print(f"Test case {i+1}: {test_case}")
        output = run_command(["node", "scripts/create_taproot_asset.js", "--params", test_case])
        
        if output:
            try:
                result = json.loads(output)
                print(f"Success: {result.get('success', False)}")
                print(f"Message: {result.get('message', 'No message')}")
                if 'files' in result:
                    print(f"Files created:")
                    for file_type, file_path in result['files'].items():
                        print(f"  - {file_type}: {file_path}")
            except json.JSONDecodeError:
                print("Error: Invalid JSON response")
        
        print("-" * 40)

def verify_mcp_config():
    """Verify the MCP configuration file"""
    print_header("Verifying MCP Configuration")
    
    config_path = os.path.join(os.getcwd(), ".cursor", "mcp.json")
    if not os.path.exists(config_path):
        print(f"Error: MCP configuration file not found at {config_path}")
        return False
    
    try:
        with open(config_path, 'r') as f:
            config = json.load(f)
            
        print(f"MCP Version: {config.get('version', 'Not specified')}")
        print(f"Tools configured: {len(config.get('tools', []))}")
        
        for i, tool in enumerate(config.get('tools', [])):
            print(f"\nTool {i+1}: {tool.get('name', 'Unnamed')}")
            print(f"Description: {tool.get('description', 'No description')}")
            print(f"Command: {tool.get('command', 'No command')}")
            
            # Check if the command file exists
            cmd_parts = tool.get('command', '').split()
            if cmd_parts:
                cmd_file = cmd_parts[0] if len(cmd_parts) > 1 and cmd_parts[0] in ['python', 'node'] else cmd_parts[0]
                cmd_path = os.path.join(os.getcwd(), *cmd_file.split('/'))
                if os.path.exists(cmd_path):
                    print(f"Command file exists: ✅")
                else:
                    print(f"Command file exists: ❌ (not found at {cmd_path})")
        
        return True
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON in MCP configuration file")
        return False
    except Exception as e:
        print(f"Error verifying MCP configuration: {e}")
        return False

def main():
    """Main function"""
    print_header("MCP Tools Test")
    
    # Verify MCP configuration
    if not verify_mcp_config():
        print("MCP configuration verification failed. Exiting.")
        sys.exit(1)
    
    # Test Bitcoin Protocol Validator
    test_bitcoin_protocol_validator()
    
    # Test Taproot Asset Creator
    test_taproot_asset_creator()
    
    print_header("MCP Tools Test Completed")

if __name__ == "__main__":
    main() 