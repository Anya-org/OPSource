#!/usr/bin/env pwsh
# Branch Management and Cleanup Script for Anya-Core Project
# Usage: ./branch_management.ps1 [action] [branch_name] [ai_label]
# Examples:
#  ./branch_management.ps1 cleanup feature/AIR-001-my-branch
#  ./branch_management.ps1 create feature/AIR-002-new-feature AIR-002
#  ./branch_management.ps1 merge feature/AIT-003-bug-fix AIT-003

param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("create", "cleanup", "merge", "status", "sync")]
    [string]$Action,
    
    [Parameter(Mandatory=$false)]
    [string]$BranchName,
    
    [Parameter(Mandatory=$false)]
    [string]$AILabel
)

$ErrorActionPreference = "Stop"
$MAIN_BRANCH = "main"
$AI_LABEL_PATTERN = "^(AIR|AIS|AIT|AIM|AIP|AIE)-\d{3}$"

function Check-Git-Status {
    $status = git status --porcelain
    if ($status) {
        Write-Host "⚠️ You have uncommitted changes. Please commit or stash them before proceeding." -ForegroundColor Yellow
        Write-Host $status
        return $false
    }
    return $true
}

function Validate-AI-Label {
    param (
        [string]$label
    )
    
    if (-not $label) {
        return $false
    }
    
    if ($label -match $AI_LABEL_PATTERN) {
        return $true
    } else {
        Write-Host "❌ Invalid AI label format. Expected format: AIR-001, AIT-002, etc." -ForegroundColor Red
        return $false
    }
}

function Create-Branch {
    param (
        [string]$branch,
        [string]$label
    )
    
    if (-not $branch) {
        Write-Host "❌ Error: Branch name is required for 'create' action" -ForegroundColor Red
        exit 1
    }
    
    # Validate AI label format if provided
    if ($label -and -not (Validate-AI-Label -label $label)) {
        exit 1
    }
    
    # Check if label is already in branch name
    $labelInBranch = $false
    if ($label) {
        $labelInBranch = $branch -match $label
    }
    
    # If label not in branch name, suggest branch name with label
    if (-not $labelInBranch -and $label) {
        $suggestedBranch = "feature/$label-" + ($branch -replace "^feature/", "")
        $useNewName = Read-Host "Would you like to use the suggested branch name with AI label: $suggestedBranch? (y/n)"
        if ($useNewName -eq "y") {
            $branch = $suggestedBranch
        }
    }
    
    # Check if we're on main branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    if ($currentBranch -ne $MAIN_BRANCH) {
        Write-Host "ℹ️ Switching to $MAIN_BRANCH branch" -ForegroundColor Cyan
        git checkout $MAIN_BRANCH
    }
    
    # Update main branch
    Write-Host "ℹ️ Updating $MAIN_BRANCH branch" -ForegroundColor Cyan
    git pull
    
    # Create new branch
    Write-Host "ℹ️ Creating new branch: $branch" -ForegroundColor Cyan
    git checkout -b $branch
    
    Write-Host "✅ Successfully created branch: $branch from $MAIN_BRANCH" -ForegroundColor Green
    
    # Suggest commit template if AI label provided
    if ($label) {
        $templateFile = "$env:TEMP\commit_template.txt"
        @"
$label: 

# Detailed explanation:
# 
# Related Items:
# - 
# 
# Changes:
# - 
"@ | Out-File -FilePath $templateFile
        
        git config --local commit.template $templateFile
        Write-Host "✅ Commit template with AI label set. Your commits will now include the $label prefix." -ForegroundColor Green
    }
    
    Write-Host "🔍 You can now make your changes and commit them to this branch" -ForegroundColor Green
}

function Cleanup-Branch {
    param (
        [string]$branch
    )
    
    if (-not $branch) {
        Write-Host "❌ Error: Branch name is required for 'cleanup' action" -ForegroundColor Red
        exit 1
    }
    
    # Get current branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    
    # Make sure we're not trying to delete the current branch
    if ($currentBranch -eq $branch) {
        Write-Host "ℹ️ Switching to $MAIN_BRANCH first..." -ForegroundColor Cyan
        git checkout $MAIN_BRANCH
    }
    
    # Check if branch exists locally
    $branchExists = git branch --list $branch
    if (-not $branchExists) {
        Write-Host "⚠️ Branch '$branch' doesn't exist locally. Skipping local deletion." -ForegroundColor Yellow
    } else {
        # Delete local branch
        Write-Host "ℹ️ Deleting local branch: $branch" -ForegroundColor Cyan
        git branch -D $branch
    }
    
    # Check if branch exists remotely
    $remoteBranchExists = git ls-remote --heads origin $branch
    if (-not $remoteBranchExists) {
        Write-Host "⚠️ Branch '$branch' doesn't exist remotely. Skipping remote deletion." -ForegroundColor Yellow
    } else {
        # Delete remote branch
        Write-Host "ℹ️ Deleting remote branch: $branch" -ForegroundColor Cyan
        git push origin --delete $branch
    }
    
    # Clear commit template if it exists
    if (Test-Path "$env:TEMP\commit_template.txt") {
        git config --local --unset commit.template
        Remove-Item "$env:TEMP\commit_template.txt" -Force
        Write-Host "ℹ️ Commit template cleared" -ForegroundColor Cyan
    }
    
    Write-Host "✅ Cleanup completed" -ForegroundColor Green
}

function Merge-Branch {
    param (
        [string]$branch,
        [string]$label
    )
    
    if (-not $branch) {
        Write-Host "❌ Error: Branch name is required for 'merge' action" -ForegroundColor Red
        exit 1
    }
    
    # Check if branch exists
    $branchExists = git branch --list $branch
    if (-not $branchExists) {
        Write-Host "❌ Error: Branch '$branch' doesn't exist" -ForegroundColor Red
        exit 1
    }
    
    # Extract AI label from branch name if not provided
    if (-not $label) {
        $branchNameMatch = $branch -match "(?:AIR|AIS|AIT|AIM|AIP|AIE)-\d{3}"
        if ($branchNameMatch) {
            $label = $Matches[0]
            Write-Host "ℹ️ Extracted AI label from branch name: $label" -ForegroundColor Cyan
        }
    }
    
    # Switch to main branch
    Write-Host "ℹ️ Switching to $MAIN_BRANCH branch" -ForegroundColor Cyan
    git checkout $MAIN_BRANCH
    
    # Update main branch
    Write-Host "ℹ️ Updating $MAIN_BRANCH branch" -ForegroundColor Cyan
    git pull
    
    # Merge feature branch
    if ($label) {
        Write-Host "ℹ️ Merging $branch into $MAIN_BRANCH with label $label" -ForegroundColor Cyan
        $mergeMessage = "$label: Merge $branch into $MAIN_BRANCH"
        git merge --no-ff $branch -m "$mergeMessage"
    } else {
        Write-Host "ℹ️ Merging $branch into $MAIN_BRANCH" -ForegroundColor Cyan
        git merge $branch
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️ Merge conflict detected. Please resolve conflicts manually and then run 'git merge --continue'" -ForegroundColor Yellow
        exit 1
    }
    
    # Push changes
    Write-Host "ℹ️ Pushing changes to remote" -ForegroundColor Cyan
    git push
    
    Write-Host "✅ Successfully merged $branch into $MAIN_BRANCH" -ForegroundColor Green
    Write-Host "ℹ️ Do you want to delete the feature branch? Run: ./branch_management.ps1 cleanup $branch" -ForegroundColor Cyan
    
    # Clear commit template if it exists
    if (Test-Path "$env:TEMP\commit_template.txt") {
        git config --local --unset commit.template
        Remove-Item "$env:TEMP\commit_template.txt" -Force
        Write-Host "ℹ️ Commit template cleared" -ForegroundColor Cyan
    }
}

function Show-Status {
    # Get current branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    Write-Host "🔍 Current branch: $currentBranch" -ForegroundColor Cyan
    
    # Extract AI label from branch name if present
    $branchNameMatch = $currentBranch -match "(?:AIR|AIS|AIT|AIM|AIP|AIE)-\d{3}"
    if ($branchNameMatch) {
        $label = $Matches[0]
        Write-Host "🏷️ AI Label: $label" -ForegroundColor Cyan
    }
    
    # Show commit template status
    if (git config --local --get commit.template) {
        Write-Host "📝 Using commit template with AI label" -ForegroundColor Cyan
    }
    
    # Show uncommitted changes
    $status = git status --porcelain
    if ($status) {
        Write-Host "📝 Uncommitted changes:" -ForegroundColor Cyan
        Write-Host $status
    } else {
        Write-Host "✅ Working directory clean" -ForegroundColor Green
    }
    
    # Show recent commits
    Write-Host "📅 Recent commits:" -ForegroundColor Cyan
    git log --oneline -n 5
    
    # Show all branches
    Write-Host "🌿 Local branches:" -ForegroundColor Cyan
    git branch
    
    # Show remote branches
    Write-Host "🌍 Remote branches:" -ForegroundColor Cyan
    git branch -r
}

function Sync-Repository {
    # Check if we have uncommitted changes
    if (-not (Check-Git-Status)) {
        Write-Host "❌ Please commit or stash your changes before syncing" -ForegroundColor Red
        exit 1
    }
    
    # Get current branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    
    # Update main branch
    Write-Host "ℹ️ Switching to $MAIN_BRANCH branch" -ForegroundColor Cyan
    git checkout $MAIN_BRANCH
    
    Write-Host "ℹ️ Updating $MAIN_BRANCH branch" -ForegroundColor Cyan
    git pull
    
    # Update all other local branches that track remote branches
    Write-Host "ℹ️ Updating all tracking branches" -ForegroundColor Cyan
    git fetch --all
    git pull --all
    
    # Switch back to original branch
    Write-Host "ℹ️ Switching back to $currentBranch" -ForegroundColor Cyan
    git checkout $currentBranch
    
    # If the current branch has a remote tracking branch, pull changes
    $hasTracking = git config --get branch.$currentBranch.merge
    if ($hasTracking) {
        Write-Host "ℹ️ Updating current branch from remote" -ForegroundColor Cyan
        git pull
    }
    
    Write-Host "✅ Repository successfully synced" -ForegroundColor Green
}

# Main execution
if (-not (Check-Git-Status)) {
    $continue = Read-Host "Do you want to continue anyway? (y/n)"
    if ($continue -ne "y") {
        exit 1
    }
}

switch ($Action) {
    "create" { Create-Branch -branch $BranchName -label $AILabel }
    "cleanup" { Cleanup-Branch -branch $BranchName }
    "merge" { Merge-Branch -branch $BranchName -label $AILabel }
    "status" { Show-Status }
    "sync" { Sync-Repository }
}
