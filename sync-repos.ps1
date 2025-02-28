# Repository Sync Script for Anya-org
# Follows Bitcoin principles of decentralization, security, and transparency

param (
    [string]$GithubToken,
    [switch]$DryRun = $false
)

# Define constants
$ORG_NAME = "Anya-org"
$TEMPLATE_DIR = Join-Path $PSScriptRoot ".github-templates"

Write-Host "`n==== Anya-org Repository Sync ====" -ForegroundColor Cyan
Write-Host "This script will sync all repositories in the Anya-org organization with standardized templates."

if (-not (Test-Path $TEMPLATE_DIR)) {
    Write-Host "Error: Template directory not found at $TEMPLATE_DIR" -ForegroundColor Red
    exit 1
}

# Prompt for GitHub token if not provided
if (-not $GithubToken) {
    $GithubToken = Read-Host "Enter GitHub token (leave blank to continue without token)"
    if ([string]::IsNullOrWhiteSpace($GithubToken)) {
        $GithubToken = $null
    }
}

Write-Host "`n==== Starting repository sync process ====" -ForegroundColor Cyan

if ($DryRun) {
    Write-Host "WARNING: Running in DRY-RUN mode. No changes will be made." -ForegroundColor Yellow
}

# Fetch repositories
Write-Host "-> Fetching repositories for $ORG_NAME..." -ForegroundColor Cyan

$headers = @{}
if ($GithubToken) {
    $headers.Add("Authorization", "token $GithubToken")
} else {
    Write-Host "WARNING: No GitHub token provided. Using public API with rate limits." -ForegroundColor Yellow
}

$page = 1
$allRepos = @()
    
do {
    $apiUrl = "https://api.github.com/orgs/$ORG_NAME/repos?per_page=100&page=$page"
    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Headers $headers -Method Get
        
        if ($response.Count -eq 0) {
            break
        }
        
        $allRepos += $response
        $page++
    } catch {
        Write-Host "Error fetching repositories: $_" -ForegroundColor Red
        exit 1
    }
} while ($response.Count -eq 100)

Write-Host "Found $($allRepos.Count) repositories" -ForegroundColor Green

# Process each repository
$successCount = 0
$failCount = 0

foreach ($repo in $allRepos) {
    $repoName = $repo.name
    $repoUrl = $repo.clone_url
    $tempDir = Join-Path $env:TEMP "anya-sync-$repoName"
    
    Write-Host "`n==== Syncing repository: $repoName ====" -ForegroundColor Cyan
    
    if (Test-Path $tempDir) {
        Remove-Item -Recurse -Force $tempDir
    }
    
    # Clone the repository
    Write-Host "-> Cloning $repoUrl to $tempDir..." -ForegroundColor Cyan
    if (-not $DryRun) {
        try {
            git clone $repoUrl $tempDir
            if ($LASTEXITCODE -ne 0) {
                Write-Host "Error: Failed to clone repository" -ForegroundColor Red
                $failCount++
                continue
            }
        } catch {
            Write-Host "Error: Failed to clone repository: $_" -ForegroundColor Red
            $failCount++
            continue
        }
    }
    
    # Create a new branch
    $branchName = "sync/github-templates-$(Get-Date -Format 'yyyyMMdd')"
    Write-Host "-> Creating branch: $branchName..." -ForegroundColor Cyan
    if (-not $DryRun) {
        try {
            Push-Location $tempDir
            git checkout -b $branchName
            if ($LASTEXITCODE -ne 0) {
                Write-Host "Error: Failed to create branch" -ForegroundColor Red
                Pop-Location
                $failCount++
                continue
            }
        } catch {
            Write-Host "Error: Failed to create branch: $_" -ForegroundColor Red
            Pop-Location
            $failCount++
            continue
        }
    }
    
    # Create necessary directories
    Write-Host "-> Creating directories..." -ForegroundColor Cyan
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
    Write-Host "-> Copying templates..." -ForegroundColor Cyan
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
            Write-Host "-> Detected Rust project, adding Rust-specific workflows..." -ForegroundColor Cyan
            Copy-Item -Path (Join-Path $TEMPLATE_DIR "rust-ci-workflow.yml") -Destination (Join-Path $workflowsDir "rust.yml") -Force
            Copy-Item -Path (Join-Path $TEMPLATE_DIR "security-scan-workflow.yml") -Destination (Join-Path $workflowsDir "security-scan.yml") -Force
            Copy-Item -Path (Join-Path $PSScriptRoot "security-scan.ps1") -Destination (Join-Path $tempDir "security-scan.ps1") -Force
        }
    }
    
    # Commit changes
    Write-Host "-> Committing changes..." -ForegroundColor Cyan
    if (-not $DryRun) {
        try {
            git -C $tempDir add .
            
            $commitMessage = "chore: Sync GitHub templates and standards

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

            git -C $tempDir commit -m $commitMessage
            
            if ($LASTEXITCODE -ne 0) {
                Write-Host "WARNING: No changes to commit or commit failed" -ForegroundColor Yellow
                Pop-Location
                continue
            }
        } catch {
            Write-Host "Error: Failed to commit changes: $_" -ForegroundColor Red
            Pop-Location
            $failCount++
            continue
        }
    }
    
    # Push changes and create PR
    Write-Host "-> Pushing changes and creating PR..." -ForegroundColor Cyan
    if (-not $DryRun) {
        try {
            if ($GithubToken) {
                # Configure git with token for push
                $repoUrlWithToken = $repoUrl -replace "https://", "https://$GithubToken@"
                git -C $tempDir push -u "$repoUrlWithToken" $branchName
            } else {
                git -C $tempDir push -u origin $branchName
            }
            
            if ($LASTEXITCODE -ne 0) {
                Write-Host "Error: Failed to push changes" -ForegroundColor Red
                Pop-Location
                $failCount++
                continue
            }
            
            # Create PR using GitHub API
            if ($GithubToken) {
                $prUrl = "https://api.github.com/repos/$ORG_NAME/$repoName/pulls"
                
                $prBodyText = "This PR syncs the repository with organization-wide templates and standards:

## Changes included:
* Add contributing guidelines
* Add code of conduct
* Add PR template
* Add issue templates
* Add workflows for CI/CD (where applicable)
* Add security scan (for Rust projects)

This ensures consistency across all Anya-org repositories and aligns with our Bitcoin principles of decentralization, security, privacy, and compatibility.

**Note:** Please review and adjust any repository-specific configurations as needed."

                $prBody = @{
                    title = "Sync GitHub templates and standards"
                    body = $prBodyText
                    head = $branchName
                    base = $repo.default_branch
                } | ConvertTo-Json
                
                $authHeaders = @{
                    Authorization = "token $GithubToken"
                    Accept = "application/vnd.github.v3+json"
                }
                
                $prResponse = Invoke-RestMethod -Uri $prUrl -Headers $authHeaders -Method Post -Body $prBody -ContentType "application/json"
                Write-Host "Created PR: $($prResponse.html_url)" -ForegroundColor Green
            } else {
                Write-Host "WARNING: GitHub token not provided. Please create PR manually." -ForegroundColor Yellow
            }
        } catch {
            Write-Host "Error: Failed to push changes or create PR: $_" -ForegroundColor Red
            $failCount++
        }
        
        Pop-Location
    }
    
    Write-Host "Repository sync complete for $repoName" -ForegroundColor Green
    $successCount++
}

# Display summary
Write-Host "`n==== Sync Summary ====" -ForegroundColor Cyan
Write-Host "Total repositories: $($allRepos.Count)" -ForegroundColor Cyan
Write-Host "Successfully synced: $successCount" -ForegroundColor Green
Write-Host "Failed to sync: $failCount" -ForegroundColor Red

Write-Host "`n==== Sync Complete ====" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "WARNING: This was a dry run. No changes were made." -ForegroundColor Yellow
    Write-Host "To perform actual changes, run the script without the -DryRun switch."
}
