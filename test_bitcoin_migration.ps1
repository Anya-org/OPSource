# Test Bitcoin Migration Script
# This script tests the Bitcoin code migration from OPSource to anya-core

# Define paths
$anyaCoreRoot = "C:\Users\bmokoka\Downloads\OPSource\anya-core"
$bitcoinDir = "$anyaCoreRoot\src\bitcoin"

Write-Host "Testing Bitcoin module migration in anya-core..." -ForegroundColor Cyan

# Function to check if a file exists
function Test-FileExists {
    param (
        [string]$filePath,
        [string]$description
    )
    
    if (Test-Path -Path $filePath) {
        Write-Host "[PASS] $description found at $filePath" -ForegroundColor Green
        return $true
    } else {
        Write-Host "[FAIL] $description not found at $filePath" -ForegroundColor Red
        return $false
    }
}

# Function to check if a directory exists
function Test-DirectoryExists {
    param (
        [string]$dirPath,
        [string]$description
    )
    
    if (Test-Path -Path $dirPath -PathType Container) {
        Write-Host "[PASS] $description directory found at $dirPath" -ForegroundColor Green
        return $true
    } else {
        Write-Host "[FAIL] $description directory not found at $dirPath" -ForegroundColor Red
        return $false
    }
}

# Function to check if a file contains specific content
function Test-FileContent {
    param (
        [string]$filePath,
        [string]$pattern,
        [string]$description
    )
    
    if (-not (Test-Path -Path $filePath)) {
        Write-Host "[FAIL] File not found at $filePath" -ForegroundColor Red
        return $false
    }
    
    $content = Get-Content -Path $filePath -Raw
    if ($content -match $pattern) {
        Write-Host "[PASS] $description found in $filePath" -ForegroundColor Green
        return $true
    } else {
        Write-Host "[FAIL] $description not found in $filePath" -ForegroundColor Red
        return $false
    }
}

# Check main directories
$tests = @(
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = $bitcoinDir; description = "Bitcoin module" } },
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = "$bitcoinDir\adapters"; description = "Adapters" } },
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = "$bitcoinDir\interface"; description = "Interface" } },
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = "$bitcoinDir\dlc"; description = "DLC" } },
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = "$bitcoinDir\cross_chain"; description = "Cross-chain" } },
    @{ Function = "Test-DirectoryExists"; Params = @{ dirPath = "$bitcoinDir\taproot"; description = "Taproot" } },
    
    # Check key files
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\mod.rs"; description = "Main Bitcoin module file" } },
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\interface\mod.rs"; description = "Bitcoin interface file" } },
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\adapters\mod.rs"; description = "Bitcoin adapters file" } },
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\adapters\rust.rs"; description = "Rust adapter file" } },
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\dlc\mod.rs"; description = "DLC module file" } },
    @{ Function = "Test-FileExists"; Params = @{ filePath = "$bitcoinDir\cross_chain\mod.rs"; description = "Cross-chain module file" } },
    
    # Check file content
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\mod.rs"; pattern = "interface"; description = "Interface module reference" } },
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\mod.rs"; pattern = "adapters"; description = "Adapters module reference" } },
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\interface\mod.rs"; pattern = "BitcoinInterface"; description = "BitcoinInterface trait" } },
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\adapters\mod.rs"; pattern = "BitcoinAdapter"; description = "BitcoinAdapter struct" } },
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\dlc\mod.rs"; pattern = "DLCContract"; description = "DLCContract struct" } },
    @{ Function = "Test-FileContent"; Params = @{ filePath = "$bitcoinDir\cross_chain\mod.rs"; pattern = "CrossChain"; description = "CrossChain functionality" } }
)

# Run all tests
$passCount = 0
$failCount = 0

foreach ($test in $tests) {
    $functionName = $test.Function
    $parameters = $test.Params
    $result = & $functionName @parameters
    if ($result) {
        $passCount++
    } else {
        $failCount++
    }
}

# Print summary
Write-Host "`nTest Summary:" -ForegroundColor Cyan
Write-Host "Passed: $passCount" -ForegroundColor Green
Write-Host "Failed: $failCount" -ForegroundColor Red

if ($failCount -eq 0) {
    Write-Host "`nAll tests passed! The Bitcoin module migration was successful." -ForegroundColor Green
    Write-Host "You can now proceed with further development and integration." -ForegroundColor Cyan
} else {
    Write-Host "`nSome tests failed. Please review the migration and fix the issues." -ForegroundColor Yellow
    Write-Host "Check the BITCOIN_MIGRATION.md file for guidance on the expected structure." -ForegroundColor Cyan
}

# Try to compile the code if cargo is available
try {
    Write-Host "`nAttempting to compile the anya-core project..." -ForegroundColor Cyan
    Push-Location $anyaCoreRoot
    cargo check --lib
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[PASS] Compilation successful!" -ForegroundColor Green
    } else {
        Write-Host "[FAIL] Compilation failed. Please check the errors and fix them." -ForegroundColor Red
    }
    Pop-Location
} catch {
    Write-Host "[SKIP] Cargo not available or compilation failed: $_" -ForegroundColor Yellow
} 