# PowerShell script to manage Git stashes
# This script helps you apply and manage stashes safely

# List all stashes
function List-Stashes {
    Write-Host "Listing all stashes:" -ForegroundColor Yellow
    git stash list
}

# Apply a stash
function Apply-Stash {
    param (
        [Parameter(Mandatory=$true)]
        [int]$StashNumber
    )
    
    # Create a temporary branch to apply the stash
    $branchName = "temp-stash-review-$StashNumber"
    
    Write-Host "Creating temporary branch '$branchName' to safely review stash..." -ForegroundColor Yellow
    git checkout -b $branchName
    
    # Apply the stash
    Write-Host "Applying stash@{$StashNumber}..." -ForegroundColor Yellow
    git stash apply stash@$StashNumber
    
    Write-Host "`nStash has been applied to temporary branch '$branchName'." -ForegroundColor Green
    Write-Host "You can now review the changes safely." -ForegroundColor Green
    Write-Host "`nOptions:" -ForegroundColor Cyan
    Write-Host "1. To keep changes: git checkout main && git merge $branchName && git branch -D $branchName" -ForegroundColor Cyan
    Write-Host "2. To discard changes: git checkout main && git branch -D $branchName" -ForegroundColor Cyan
    Write-Host "3. To drop the stash: git stash drop stash@$StashNumber" -ForegroundColor Cyan
}

# Drop a stash
function Drop-Stash {
    param (
        [Parameter(Mandatory=$true)]
        [int]$StashNumber
    )
    
    Write-Host "Dropping stash@{$StashNumber}..." -ForegroundColor Yellow
    git stash drop stash@$StashNumber
    
    Write-Host "Stash has been dropped." -ForegroundColor Green
}

# Show stash contents
function Show-Stash {
    param (
        [Parameter(Mandatory=$true)]
        [int]$StashNumber
    )
    
    Write-Host "Showing contents of stash@{$StashNumber}:" -ForegroundColor Yellow
    git stash show -p stash@$StashNumber
}

# Main menu
function Show-Menu {
    Write-Host "`n===== Git Stash Manager =====" -ForegroundColor Magenta
    Write-Host "1: List all stashes" -ForegroundColor Cyan
    Write-Host "2: Apply a stash to a temporary branch" -ForegroundColor Cyan
    Write-Host "3: Show stash contents" -ForegroundColor Cyan
    Write-Host "4: Drop a stash" -ForegroundColor Cyan
    Write-Host "5: Exit" -ForegroundColor Cyan
    Write-Host "===========================" -ForegroundColor Magenta
}

# Main loop
$continue = $true
while ($continue) {
    Show-Menu
    $choice = Read-Host "Enter your choice (1-5)"
    
    switch ($choice) {
        1 { List-Stashes }
        2 { 
            $stashNum = Read-Host "Enter stash number (0, 1, 2, etc.)"
            Apply-Stash -StashNumber $stashNum 
        }
        3 { 
            $stashNum = Read-Host "Enter stash number (0, 1, 2, etc.)"
            Show-Stash -StashNumber $stashNum 
        }
        4 { 
            $stashNum = Read-Host "Enter stash number (0, 1, 2, etc.)"
            Drop-Stash -StashNumber $stashNum 
        }
        5 { $continue = $false }
        default { Write-Host "Invalid choice, please try again." -ForegroundColor Red }
    }
}

Write-Host "`nExiting stash manager. Goodbye!" -ForegroundColor Green
