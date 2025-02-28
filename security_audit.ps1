# Security Audit Script for Bitcoin-focused Python Projects
# This script helps audit dependencies for security vulnerabilities
# and performs basic security checks on the codebase

Write-Host "Starting Security Audit..." -ForegroundColor Green

# Check for pip security updates
Write-Host "`nChecking for outdated packages with known vulnerabilities..." -ForegroundColor Cyan
pip list --outdated

# Install and run safety to check for known vulnerabilities
Write-Host "`nInstalling safety tool for security checks..." -ForegroundColor Cyan
pip install safety --quiet
Write-Host "`nRunning safety check on requirements..." -ForegroundColor Cyan
safety check -r requirements.txt

# Run bandit for Python security issues
Write-Host "`nInstalling bandit for code security scanning..." -ForegroundColor Cyan
pip install bandit --quiet
Write-Host "`nRunning bandit security scan on codebase..." -ForegroundColor Cyan
bandit -r . -x ./tests,./.git

# Check for cryptographic implementations
Write-Host "`nChecking for insecure cryptographic implementations..." -ForegroundColor Cyan
$insecurePatterns = @(
    "MD5",
    "SHA-1",
    "random.random\(", 
    "random.randint\(",
    "ECB mode"
)

foreach ($pattern in $insecurePatterns) {
    Write-Host "Searching for potential insecure pattern: $pattern" -ForegroundColor Yellow
    $results = Get-ChildItem -Path . -Recurse -Include *.py, *.rs | Select-String -Pattern $pattern
    if ($results) {
        Write-Host "FOUND POTENTIAL INSECURE PATTERN: $pattern" -ForegroundColor Red
        $results | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
}

# Look for hardcoded secrets and API keys
Write-Host "`nChecking for potentially hardcoded secrets..." -ForegroundColor Cyan
$secretPatterns = @(
    "api_key",
    "apikey",
    "password",
    "secret",
    "token"
)

foreach ($pattern in $secretPatterns) {
    Write-Host "Searching for potential hardcoded secrets: $pattern" -ForegroundColor Yellow
    $results = Get-ChildItem -Path . -Recurse -Include *.py, *.rs, *.js, *.json | 
               Select-String -Pattern "(?i)$pattern.*['\"""][a-zA-Z0-9_\-]{16,}['\"""]"
    if ($results) {
        Write-Host "POTENTIAL HARDCODED SECRET FOUND: $pattern" -ForegroundColor Red
        $results | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
}

# Bitcoin-specific checks
Write-Host "`nRunning Bitcoin-specific security checks..." -ForegroundColor Cyan
$bitcoinPatterns = @(
    "testnet",
    "mainnet",
    "bitcoin_rpc",
    "private_key",
    "mnemonic"
)

foreach ($pattern in $bitcoinPatterns) {
    $results = Get-ChildItem -Path . -Recurse -Include *.py, *.rs | Select-String -Pattern $pattern
    if ($results) {
        Write-Host "Bitcoin-specific pattern found: $pattern (manually verify security)" -ForegroundColor Yellow
        $results | ForEach-Object { Write-Host "  $_" -ForegroundColor Yellow }
    }
}

Write-Host "`nSecurity audit completed. Review findings above and address any issues found." -ForegroundColor Green
Write-Host "Remember to regularly check Dependabot alerts and keep dependencies updated." -ForegroundColor Green
