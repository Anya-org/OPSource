# Security scanning script for OPSource Rust projects
# Follows Bitcoin principles of security and decentralization

Write-Host "Starting security scan for OPSource Rust projects..." -ForegroundColor Cyan

# Check if required tools are installed
function Check-ToolInstalled {
    param (
        [string]$Tool,
        [string]$InstallCommand
    )
    
    if (-not (Get-Command $Tool -ErrorAction SilentlyContinue)) {
        Write-Host "❌ $Tool is not installed. Installing..." -ForegroundColor Yellow
        Invoke-Expression $InstallCommand
        if (-not $?) {
            Write-Host "Failed to install $Tool" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "✅ $Tool is already installed" -ForegroundColor Green
    }
}

Check-ToolInstalled "cargo-outdated" "cargo install cargo-outdated"
Check-ToolInstalled "cargo-deny" "cargo install cargo-deny@0.17.0"

# Get all Cargo.toml files except those in env/ directory
$cargoFiles = Get-ChildItem -Path . -Include "Cargo.toml" -Recurse | 
    Where-Object { $_.FullName -notlike "*\env\*" }

$exitCode = 0
$totalIssues = 0
$criticalIssues = 0

Write-Host "`nFound $($cargoFiles.Count) Rust projects to scan" -ForegroundColor Cyan
Write-Host "--------------------------------------------------" -ForegroundColor Cyan

foreach ($cargoFile in $cargoFiles) {
    $projectDir = $cargoFile.DirectoryName
    $projectName = Split-Path $projectDir -Leaf
    
    Write-Host "`n📦 Scanning project: $projectName" -ForegroundColor Magenta
    Write-Host "  Location: $projectDir"
    
    # Skip cargo if it's a large workspace (like the root)
    if ($projectDir -eq (Get-Location).Path) {
        Write-Host "  Skipping root workspace scan" -ForegroundColor Yellow
        continue
    }
    
    # Check for outdated dependencies
    Write-Host "`n  🔍 Checking for outdated dependencies..." -ForegroundColor Cyan
    Set-Location $projectDir
    try {
        $outdatedOutput = cargo outdated --exit-code 1 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  ✅ No outdated dependencies found" -ForegroundColor Green
        } else {
            Write-Host "  ⚠️ Outdated dependencies found:" -ForegroundColor Yellow
            Write-Host $outdatedOutput
            $totalIssues++
        }
    } catch {
        Write-Host "  ❌ Error checking outdated dependencies: $_" -ForegroundColor Red
    }
    
    # Return to the original directory
    Set-Location (Split-Path -Parent $PSScriptRoot)
}

# Summary
Write-Host "`n--------------------------------------------------" -ForegroundColor Cyan
Write-Host "Security scan complete!" -ForegroundColor Cyan
Write-Host "Total issues found: $totalIssues" -ForegroundColor $(if ($totalIssues -gt 0) { "Yellow" } else { "Green" })
Write-Host "Critical issues found: $criticalIssues" -ForegroundColor $(if ($criticalIssues -gt 0) { "Red" } else { "Green" })

if ($criticalIssues -gt 0) {
    Write-Host "`n❌ Security scan failed due to critical issues" -ForegroundColor Red
    exit 1
} elseif ($totalIssues -gt 0) {
    Write-Host "`n⚠️ Security scan completed with warnings" -ForegroundColor Yellow
    exit 0
} else {
    Write-Host "`n✅ Security scan completed successfully with no issues found" -ForegroundColor Green
    exit 0
}
