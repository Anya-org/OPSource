#!/usr/bin/env pwsh
# Read First Implementation Branch Management Script
# Usage: ./read_first_branch.ps1 [action] [component_name]
# Examples:
#  ./read_first_branch.ps1 create web5-service
#  ./read_first_branch.ps1 merge read-first-web5-service

param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("create", "pr", "merge", "metrics")]
    [string]$Action,
    
    [Parameter(Mandatory=$false)]
    [string]$ComponentName
)

$ErrorActionPreference = "Stop"
$MAIN_BRANCH = "main"
$READ_FIRST_LABEL = "AIP-001"
$PR_TEMPLATE_PATH = ".github/PULL_REQUEST_TEMPLATE/read_first_template.md"

function Create-ReadFirst-Branch {
    param (
        [string]$component
    )
    
    if (-not $component) {
        Write-Host "❌ Error: Component name is required for 'create' action" -ForegroundColor Red
        exit 1
    }
    
    # Format branch name
    $branchName = "feature/$READ_FIRST_LABEL-read-first-$component"
    
    # Create branch using the branch_management script
    Write-Host "ℹ️ Creating Read First branch for component: $component" -ForegroundColor Cyan
    & "$PSScriptRoot\branch_management.ps1" create $branchName $READ_FIRST_LABEL
    
    # Create standard directory structure for Read First implementation
    # This will depend on your project structure, adjust as needed
    $readFirstContent = @"
// Read First implementation for $component
// Implements $READ_FIRST_LABEL: Read First Always Pattern

/**
 * ReadFirstManager ensures all operations read current state before making changes.
 * This helps prevent race conditions and maintain data integrity.
 */
class ReadFirst${component}Manager {
  // Implementation details
  
  /**
   * Tracks metrics related to Read First compliance
   */
  final readFirstMetrics = {
    'read_count': 0,
    'write_count': 0,
    'violation_count': 0,
  };
  
  /**
   * Creates a new record, enforcing the Read First principle.
   * Reads the current state before creating to prevent duplicates.
   */
  Future<Record> createRecord(CreateRecordOptions options) async {
    // Read First: Query similar records first
    await queryRecords({
      // Query parameters based on options
    });
    
    // Track metrics
    readFirstMetrics['read_count']++;
    readFirstMetrics['write_count']++;
    
    // Proceed with creation
    return await _underlyingService.createRecord(options);
  }
  
  // Additional methods following the same pattern
  
  /**
   * Returns metrics about Read First compliance
   */
  Map<String, dynamic> getReadFirstMetrics() {
    final total = readFirstMetrics['read_count'] + readFirstMetrics['write_count'];
    final complianceRate = total > 0 
      ? ((readFirstMetrics['read_count'] / total) * 100).toStringAsFixed(2)
      : '100.00';
      
    return {
      'read_count': readFirstMetrics['read_count'],
      'write_count': readFirstMetrics['write_count'],
      'violation_count': readFirstMetrics['violation_count'],
      'compliance_rate': complianceRate,
    };
  }
}
"@
    
    # Suggest file creation (not actually creating since we don't know the structure)
    Write-Host "ℹ️ Suggested Read First implementation for $component:" -ForegroundColor Cyan
    Write-Host $readFirstContent
    
    # Add example commit message
    Write-Host "ℹ️ Example commit message:" -ForegroundColor Cyan
    Write-Host "$READ_FIRST_LABEL: Implement Read First Always pattern for $component" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "- Add ReadFirst${component}Manager with metrics tracking" -ForegroundColor Yellow
    Write-Host "- Enforce read-before-write in all operations" -ForegroundColor Yellow
    Write-Host "- Add tests to verify compliance" -ForegroundColor Yellow
    
    Write-Host "✅ Ready to implement Read First for $component" -ForegroundColor Green
}

function Prepare-PR {
    # Get current branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    
    # Check if branch follows Read First naming convention
    if (-not ($currentBranch -match "$READ_FIRST_LABEL")) {
        Write-Host "⚠️ Current branch does not follow Read First naming convention. Continuing anyway." -ForegroundColor Yellow
    }
    
    # Extract component name from branch
    $component = ""
    if ($currentBranch -match "read-first-(.+)") {
        $component = $Matches[1]
    } else {
        $component = Read-Host "Enter the component name for this Read First implementation"
    }
    
    # Read PR template
    $prTemplate = Get-Content -Path $PR_TEMPLATE_PATH -Raw
    
    # Replace placeholders
    $prTemplate = $prTemplate -replace "\[Component Name\]", $component
    
    # Save to a temporary file
    $tempFile = "$env:TEMP\read_first_pr_$component.md"
    $prTemplate | Out-File -FilePath $tempFile
    
    # Open the file
    Start-Process $tempFile
    
    Write-Host "✅ PR template prepared for $component" -ForegroundColor Green
    Write-Host "ℹ️ Edit the template at $tempFile and use it when creating your PR" -ForegroundColor Cyan
}

function Merge-ReadFirst-Branch {
    param (
        [string]$branch
    )
    
    if (-not $branch) {
        # Get current branch
        $branch = git rev-parse --abbrev-ref HEAD
    }
    
    # Ensure branch has AIP-001 label
    if (-not ($branch -match "$READ_FIRST_LABEL")) {
        $addLabel = Read-Host "Branch does not have the $READ_FIRST_LABEL label. Add it? (y/n)"
        if ($addLabel -eq "y") {
            $branch = $branch -replace "^feature/", "feature/$READ_FIRST_LABEL-"
            
            # Rename branch
            $currentBranch = git rev-parse --abbrev-ref HEAD
            if ($currentBranch -eq $branch.Replace("$READ_FIRST_LABEL-", "")) {
                git branch -m $branch
                Write-Host "ℹ️ Renamed branch to $branch" -ForegroundColor Cyan
            }
        }
    }
    
    # Use branch_management script to merge
    & "$PSScriptRoot\branch_management.ps1" merge $branch $READ_FIRST_LABEL
}

function Check-ReadFirst-Metrics {
    # Check if log file exists in standard locations
    $potentialLogFiles = @(
        "logs/read_first_metrics.log",
        "var/log/read_first_metrics.log",
        "tmp/read_first.log"
    )
    
    $logFile = $null
    foreach ($file in $potentialLogFiles) {
        if (Test-Path $file) {
            $logFile = $file
            break
        }
    }
    
    if ($logFile) {
        # Read metrics from log file
        $metrics = Get-Content $logFile | Where-Object { $_ -match "ReadFirstMetrics" }
        
        # Display metrics
        Write-Host "📊 Read First Metrics from $logFile:" -ForegroundColor Cyan
        foreach ($metric in $metrics) {
            Write-Host $metric -ForegroundColor Yellow
        }
    } else {
        Write-Host "ℹ️ No Read First metrics log file found. Checking for in-memory metrics..." -ForegroundColor Cyan
        
        # Suggest command to check metrics in code
        Write-Host "Add this code to check metrics in your application:" -ForegroundColor Yellow
        Write-Host "final metrics = yourService.getReadFirstMetrics();" -ForegroundColor Yellow
        Write-Host "print('Read count: \${metrics[\"read_count\"]}');" -ForegroundColor Yellow
        Write-Host "print('Write count: \${metrics[\"write_count\"]}');" -ForegroundColor Yellow
        Write-Host "print('Compliance rate: \${metrics[\"compliance_rate\"]}%');" -ForegroundColor Yellow
    }
}

# Main execution
switch ($Action) {
    "create" { Create-ReadFirst-Branch -component $ComponentName }
    "pr" { Prepare-PR }
    "merge" { Merge-ReadFirst-Branch -branch $ComponentName }
    "metrics" { Check-ReadFirst-Metrics }
}
