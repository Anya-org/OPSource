# Repository Sync Script for Anya-org
# Follows Bitcoin principles of decentralization, security, and transparency

param (
    [string]$GithubToken,
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"
$ORG_NAME = "Anya-org"
$TEMPLATE_DIR = Join-Path $PSScriptRoot ".github-templates"

# Colors for output
$COLOR_INFO = "Cyan"
$COLOR_SUCCESS = "Green"
$COLOR_WARNING = "Yellow"
$COLOR_ERROR = "Red"

function Write-Title {
    param (
        [string]$Title
    )
    Write-Host "`n==== $Title ====" -ForegroundColor $COLOR_INFO
}

function Write-Step {
    param (
        [string]$Step
    )
    Write-Host "→ $Step" -ForegroundColor $COLOR_INFO
}

function Write-Success {
    param (
        [string]$Message
    )
    Write-Host "✓ $Message" -ForegroundColor $COLOR_SUCCESS
}

function Write-Warning {
    param (
        [string]$Message
    )
    Write-Host "⚠ $Message" -ForegroundColor $COLOR_WARNING
}

function Write-Error {
    param (
        [string]$Message
    )
    Write-Host "✗ $Message" -ForegroundColor $COLOR_ERROR
}

function Get-Repositories {
    Write-Step "Fetching repositories for $ORG_NAME..."
    
    if (-not $GithubToken) {
        Write-Warning "No GitHub token provided. Using public API with rate limits."
        $headers = @{}
    } else {
        $headers = @{
            Authorization = "token $GithubToken"
        }
    }
    
    $page = 1
    $allRepos = @()
    
    do {
        $apiUrl = "https://api.github.com/orgs/$ORG_NAME/repos?per_page=100" + "&page=$page"
        $response = Invoke-RestMethod -Uri $apiUrl -Headers $headers -Method Get
        
        if ($response.Count -eq 0) {
            break
        }
        
        $allRepos += $response
        $page++
    } while ($response.Count -eq 100)
    
    Write-Success "Found $($allRepos.Count) repositories"
    return $allRepos
}

function Sync-Repository {
    param (
        [PSObject]$Repo
    )
    
    $repoName = $Repo.name
    $repoUrl = $Repo.clone_url
    $tempDir = Join-Path $env:TEMP "anya-sync-$repoName"
    
    Write-Title "Syncing repository: $repoName"
    
    if (Test-Path $tempDir) {
        Remove-Item -Recurse -Force $tempDir
    }
    
    # Clone the repository
    Write-Step "Cloning $repoUrl to $tempDir..."
    if (-not $DryRun) {
        & git clone $repoUrl $tempDir
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to clone repository"
            return $false
        }
    }
    
    # Create a new branch
    $branchName = "sync/github-templates-$(Get-Date -Format 'yyyyMMdd')"
    Write-Step "Creating branch: $branchName..."
    if (-not $DryRun) {
        Push-Location $tempDir
        & git checkout -b $branchName
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to create branch"
            Pop-Location
            return $false
        }
    }
    
    # Create necessary directories
    Write-Step "Creating directories..."
    if (-not $DryRun) {
        $githubDir = Join-Path $tempDir ".github"
        $issueTemplateDir = Join-Path $githubDir "ISSUE_TEMPLATE"
        $workflowsDir = Join-Path $githubDir "workflows"
        
        if (-not (Test-Path $githubDir)) {
            New-Item -ItemType Directory -Path $githubDir | Out-Null
        }
        
        if (-not (Test-Path $issueTemplateDir)) {
            New-Item -ItemType Directory -Path $issueTemplateDir | Out-Null
        }
        
        if (-not (Test-Path $workflowsDir)) {
            New-Item -ItemType Directory -Path $workflowsDir | Out-Null
        }
    }
    
    # Copy templates
    Write-Step "Copying templates..."
    if (-not $DryRun) {
        # Root files
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "CONTRIBUTING-template.md") -Destination (Join-Path $tempDir "CONTRIBUTING.md") -Force
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "CODE_OF_CONDUCT-template.md") -Destination (Join-Path $tempDir "CODE_OF_CONDUCT.md") -Force
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "MIT-LICENSE-template.txt") -Destination (Join-Path $tempDir "LICENSE") -Force
        
        # GitHub directory
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "PR-TEMPLATE.md") -Destination (Join-Path $githubDir "PULL_REQUEST_TEMPLATE.md") -Force
        
        # Issue templates
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "bug_report.md") -Destination (Join-Path $issueTemplateDir "bug_report.md") -Force
        Copy-Item -Path (Join-Path $TEMPLATE_DIR "feature_request.md") -Destination (Join-Path $issueTemplateDir "feature_request.md") -Force
        
        # Workflows for Rust projects
        if (Test-Path (Join-Path $tempDir "Cargo.toml")) {
            Write-Step "Detected Rust project, adding Rust-specific workflows..."
            Copy-Item -Path (Join-Path $TEMPLATE_DIR "rust-ci-workflow.yml") -Destination (Join-Path $workflowsDir "rust.yml") -Force
            Copy-Item -Path (Join-Path $TEMPLATE_DIR "security-scan-workflow.yml") -Destination (Join-Path $workflowsDir "security-scan.yml") -Force
            Copy-Item -Path (Join-Path $PSScriptRoot "security-scan.ps1") -Destination (Join-Path $tempDir "security-scan.ps1") -Force
        }
    }
    
    # Commit changes
    Write-Step "Committing changes..."
    if (-not $DryRun) {
        & git add .
        & git commit -m "chore: Sync GitHub templates and standards

Apply organization-wide templates:
* Add contributing guidelines
* Add code of conduct
* Add PR template
* Add issue templates
* Add workflows for CI/CD
* Add security scan

This ensures consistency across all Anya-org repositories and
aligns with our Bitcoin principles of decentralization,
security, privacy, and compatibility."
        
        if ($LASTEXITCODE -ne 0) {
            Write-Warning "No changes to commit or commit failed"
            Pop-Location
            return $false
        }
    }
    
    # Push changes and create PR
    Write-Step "Pushing changes and creating PR..."
    if (-not $DryRun) {
        if ($GithubToken) {
            # Configure git with token for push
            $repoUrlWithToken = $repoUrl -replace "https://", "https://$GithubToken@"
            & git push -u "$repoUrlWithToken" $branchName
        } else {
            & git push -u origin $branchName
        }
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to push changes"
            Pop-Location
            return $false
        }
        
        # Create PR using GitHub API
        if ($GithubToken) {
            $prUrl = "https://api.github.com/repos/$ORG_NAME/$repoName/pulls"
            $prBody = @{
                title = "Sync GitHub templates and standards"
                body = "This PR syncs the repository with organization-wide templates and standards:

## Changes included:
* Add contributing guidelines
* Add code of conduct
* Add PR template
* Add issue templates
* Add workflows for CI/CD (where applicable)
* Add security scan (for Rust projects)

This ensures consistency across all Anya-org repositories and aligns with our Bitcoin principles of decentralization, security, privacy, and compatibility.

**Note:** Please review and adjust any repository-specific configurations as needed."
                head = $branchName
                base = $Repo.default_branch
            } | ConvertTo-Json
            
            $headers = @{
                Authorization = "token $GithubToken"
                Accept = "application/vnd.github.v3+json"
            }
            
            try {
                $prResponse = Invoke-RestMethod -Uri $prUrl -Headers $headers -Method Post -Body $prBody -ContentType "application/json"
                Write-Success "Created PR: $($prResponse.html_url)"
            } catch {
                Write-Error "Failed to create PR: $_"
            }
        } else {
            Write-Warning "GitHub token not provided. Please create PR manually."
        }
        
        Pop-Location
    }
    
    Write-Success "Repository sync complete for $repoName"
    return $true
}

function Sync-AllRepositories {
    Write-Title "Starting repository sync process"
    
    if ($DryRun) {
        Write-Warning "Running in DRY-RUN mode. No changes will be made."
    }
    
    $repos = Get-Repositories
    $successCount = 0
    $failCount = 0
    
    foreach ($repo in $repos) {
        $success = Sync-Repository -Repo $repo
        
        if ($success) {
            $successCount++
        } else {
            $failCount++
        }
    }
    
    Write-Title "Sync Summary"
    Write-Host "Total repositories: $($repos.Count)" -ForegroundColor $COLOR_INFO
    Write-Host "Successfully synced: $successCount" -ForegroundColor $COLOR_SUCCESS
    Write-Host "Failed to sync: $failCount" -ForegroundColor $COLOR_ERROR
}

# Main script execution
Write-Title "Anya-org Repository Sync"
Write-Host "This script will sync all repositories in the Anya-org organization with standardized templates."

if (-not (Test-Path $TEMPLATE_DIR)) {
    Write-Error "Template directory not found at $TEMPLATE_DIR"
    exit 1
}

# Prompt for GitHub token if not provided
if (-not $GithubToken) {
    $GithubToken = Read-Host "Enter GitHub token (leave blank to continue without token)"
    if ([string]::IsNullOrWhiteSpace($GithubToken)) {
        $GithubToken = $null
    }
}

Sync-AllRepositories

Write-Title "Sync Complete"
if ($DryRun) {
    Write-Warning "This was a dry run. No changes were made."
    Write-Host "To perform actual changes, run the script without the -DryRun switch."
}
