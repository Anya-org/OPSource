# Bitcoin Code Migration Script
# This script migrates Bitcoin code from OPSource to anya-core

# Define source and destination paths
$sourceRoot = "C:\Users\bmokoka\Downloads\OPSource\src\bitcoin"
$destRoot = "C:\Users\bmokoka\Downloads\OPSource\anya-core\src\bitcoin"

# Create backup of destination directory
$backupDir = "C:\Users\bmokoka\Downloads\OPSource\anya-core\src\bitcoin_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
Write-Host "Creating backup of anya-core Bitcoin module at $backupDir"
Copy-Item -Path $destRoot -Destination $backupDir -Recurse -Force

# Function to copy and adapt files
function Copy-AndAdaptFile {
    param (
        [string]$sourcePath,
        [string]$destPath
    )
    
    # Create destination directory if it doesn't exist
    $destDir = Split-Path -Path $destPath -Parent
    if (-not (Test-Path -Path $destDir)) {
        New-Item -Path $destDir -ItemType Directory -Force | Out-Null
    }
    
    # Read source file content
    $content = Get-Content -Path $sourcePath -Raw
    
    # Adapt content for anya-core
    # Replace OPSource-specific imports with anya-core imports
    $content = $content -replace "use crate::config", "use crate::bitcoin::config"
    $content = $content -replace "use crate::bitcoin::interface", "use crate::bitcoin::interface"
    
    # Add anya-core specific header
    $header = "// Migrated from OPSource to anya-core
// This file was automatically migrated as part of the Rust-only implementation
// Original file: $sourcePath
"
    $content = $header + $content
    
    # Write adapted content to destination
    Set-Content -Path $destPath -Value $content -Force
    
    Write-Host "Migrated: $sourcePath -> $destPath"
}

# Create necessary directories in destination
$directories = @(
    "cross_chain",
    "dlc",
    "taproot",
    "interface",
    "adapters"
)

foreach ($dir in $directories) {
    $path = Join-Path -Path $destRoot -ChildPath $dir
    if (-not (Test-Path -Path $path)) {
        New-Item -Path $path -ItemType Directory -Force | Out-Null
        Write-Host "Created directory: $path"
    }
}

# Copy main module file
Copy-AndAdaptFile -sourcePath "$sourceRoot\mod.rs" -destPath "$destRoot\mod.rs"

# Copy DLC module
$dlcFiles = Get-ChildItem -Path "$sourceRoot\dlc" -Recurse -File
foreach ($file in $dlcFiles) {
    $relativePath = $file.FullName.Substring("$sourceRoot\dlc\".Length)
    $destPath = Join-Path -Path "$destRoot\dlc" -ChildPath $relativePath
    Copy-AndAdaptFile -sourcePath $file.FullName -destPath $destPath
}

# Copy Cross-Chain module
$crossChainFiles = Get-ChildItem -Path "$sourceRoot\cross_chain" -Recurse -File
foreach ($file in $crossChainFiles) {
    $relativePath = $file.FullName.Substring("$sourceRoot\cross_chain\".Length)
    $destPath = Join-Path -Path "$destRoot\cross_chain" -ChildPath $relativePath
    Copy-AndAdaptFile -sourcePath $file.FullName -destPath $destPath
}

# Copy Taproot module
$taprootFiles = Get-ChildItem -Path "$sourceRoot\taproot" -Recurse -File
foreach ($file in $taprootFiles) {
    $relativePath = $file.FullName.Substring("$sourceRoot\taproot\".Length)
    $destPath = Join-Path -Path "$destRoot\taproot" -ChildPath $relativePath
    Copy-AndAdaptFile -sourcePath $file.FullName -destPath $destPath
}

# Copy interface files
Copy-AndAdaptFile -sourcePath "$sourceRoot\interface.rs" -destPath "$destRoot\interface\mod.rs"
Copy-AndAdaptFile -sourcePath "$sourceRoot\adapter.rs" -destPath "$destRoot\adapters\mod.rs"
Copy-AndAdaptFile -sourcePath "$sourceRoot\rust.rs" -destPath "$destRoot\adapters\rust.rs"

# Update main mod.rs to include new modules
$mainModContent = Get-Content -Path "$destRoot\mod.rs" -Raw
if (-not $mainModContent.Contains("pub mod interface")) {
    $mainModContent = $mainModContent -replace "mod wallet;", "mod wallet;`nmod interface;`nmod adapters;"
    Set-Content -Path "$destRoot\mod.rs" -Value $mainModContent -Force
    Write-Host "Updated main mod.rs to include new modules"
}

Write-Host "Migration completed successfully!"
Write-Host "Please review the migrated code and make any necessary adjustments."
Write-Host "A backup of the original anya-core Bitcoin module is available at: $backupDir" 