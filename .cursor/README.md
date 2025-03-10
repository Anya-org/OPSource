# MCP Configuration for Bitcoin Development

This directory contains the Model Context Protocol (MCP) configuration for the Cursor AI assistant. The MCP allows the AI to use custom tools specific to Bitcoin development according to the Bitcoin Development Framework v2.5.

## Available Tools

### Bitcoin Protocol Validator

This tool validates Bitcoin protocol compliance according to BIP standards.

**Example Usage:**
```
Verify this transaction follows BIP-341 taproot standard: 
tr(KEY,{SILENT_LEAF})
```

### Taproot Asset Creator

Creates Taproot assets with proper metadata according to the project standards.

**Example Usage:**
```
Create asset named TokenX with supply 1000000
```

### Bitcoin Security Audit

Runs security audit on Bitcoin code according to compliance checklist.

**Example Usage:**
```
Check this DLC implementation for timing vulnerabilities:
function verifySignature(signature, message) {
  // Code to check
}
```

## How It Works

1. The MCP configuration in `mcp.json` defines tools that can be invoked by the AI assistant.
2. Each tool maps to a script in the project that performs a specific function.
3. The AI can use these tools to help with development tasks like protocol validation, asset creation, and security checks.

## Configuration

The MCP configuration uses the stdio protocol format as specified in the [Cursor documentation](https://docs.cursor.com/context/model-context-protocol). If you need to modify or add tools, update the `mcp.json` file and ensure the corresponding scripts are available.

## Security Note

The MCP tools run on your local machine. Only add trusted scripts and tools to the configuration.

## Hexagonal Architecture Compliance

All tools follow the project's hexagonal architecture requirements:
- Clear separation of core logic from adapters
- BIP-agnostic implementation when possible
- Protocol-level interoperability
- Adherence to the AI labelling guidelines

## Related Documentation

- [Bitcoin Development Framework v2.5](../docs/bitcoin-framework.md)
- [AI Labelling Guidelines](../docs/ai-labelling.md)
- [BIP Support Matrix](../docs/bip-support-matrix.md) 