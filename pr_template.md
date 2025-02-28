## Security Fix: Cryptography Package Update

This PR addresses critical and high severity security vulnerabilities by:

1. **Updating the cryptography package from 44.0.1 to 44.0.2** to fix a potential remote code execution vulnerability
2. **Enhancing Dependabot configuration** for more aggressive security update handling
3. **Adding security documentation** with the SECURITY_FIXES.md file

### Changes

- Updated cryptography to v44.0.2 in all requirements files
- Enhanced Dependabot configuration with:
  - More aggressive security update settings
  - Additional package ecosystem monitoring
  - Automatic labeling for security PRs
- Added security documentation and monitoring guidance

### Security Impact

These changes address the critical vulnerability notification from GitHub and establish a better framework for managing security updates in the future.

### Testing

The cryptography update maintains API compatibility while fixing the security issues.

### Related Issues

Addresses the 6 vulnerabilities currently detected by GitHub's security scanning.

### Bitcoin-Specific Security Considerations

These changes improve our compliance with Bitcoin development security best practices by ensuring that our cryptographic dependencies meet the highest security standards, which is essential for Bitcoin-related operations.
