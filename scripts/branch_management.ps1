#!/usr/bin/env pwsh
# Branch Management and Cleanup Script for Anya-Core Project
# Usage: ./branch_management.ps1 [action] [branch_name]
# Examples:
#  ./branch_management.ps1 cleanup feature/my-branch
#  ./branch_management.ps1 create feature/new-feature
#  ./branch_management.ps1 merge feature/my-branch

param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("create", "cleanup", "merge", "status", "sync")]
    [string]$Action,
    
    [Parameter(Mandatory=$false)]
    [string]$BranchName
)

$ErrorActionPreference = "Stop"
$MAIN_BRANCH = "main"

function Check-Git-Status {
    $status = git status --porcelain
    if ($status) {
        Write-Host "⚠️ You have uncommitted changes. Please commit or stash them before proceeding." -ForegroundColor Yellow
        Write-Host $status
        return $false
    }
    return $true
}

function Create-Branch {
    param (
        [string]$branch
    )
    
    if (-not $branch) {
        Write-Host "❌ Error: Branch name is required for 'create' action" -ForegroundColor Red
        exit 1
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
    
    Write-Host "✅ Cleanup completed" -ForegroundColor Green
}

function Merge-Branch {
    param (
        [string]$branch
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
    
    # Switch to main branch
    Write-Host "ℹ️ Switching to $MAIN_BRANCH branch" -ForegroundColor Cyan
    git checkout $MAIN_BRANCH
    
    # Update main branch
    Write-Host "ℹ️ Updating $MAIN_BRANCH branch" -ForegroundColor Cyan
    git pull
    
    # Merge feature branch
    Write-Host "ℹ️ Merging $branch into $MAIN_BRANCH" -ForegroundColor Cyan
    git merge $branch
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️ Merge conflict detected. Please resolve conflicts manually and then run 'git merge --continue'" -ForegroundColor Yellow
        exit 1
    }
    
    # Push changes
    Write-Host "ℹ️ Pushing changes to remote" -ForegroundColor Cyan
    git push
    
    Write-Host "✅ Successfully merged $branch into $MAIN_BRANCH" -ForegroundColor Green
    Write-Host "ℹ️ Do you want to delete the feature branch? Run: ./branch_management.ps1 cleanup $branch" -ForegroundColor Cyan
}

function Show-Status {
    # Get current branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    Write-Host "🔍 Current branch: $currentBranch" -ForegroundColor Cyan
    
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
    "create" { Create-Branch -branch $BranchName }
    "cleanup" { Cleanup-Branch -branch $BranchName }
    "merge" { Merge-Branch -branch $BranchName }
    "status" { Show-Status }
    "sync" { Sync-Repository }
}
