# PowerShell script to clean up branches after PR merge
# Run this script after the fix/security-vulnerabilities PR has been merged to main

# Ensure we're on main branch
Write-Host "Switching to main branch..." -ForegroundColor Yellow
git checkout main

# Pull latest changes to ensure we have the merged PR
Write-Host "Pulling latest changes..." -ForegroundColor Yellow
git pull origin main

# Delete the local fix/security-vulnerabilities branch
Write-Host "Deleting local fix/security-vulnerabilities branch..." -ForegroundColor Yellow
git branch -D fix/security-vulnerabilities

# Delete the remote fix/security-vulnerabilities branch
Write-Host "Deleting remote fix/security-vulnerabilities branch..." -ForegroundColor Yellow
git push origin --delete fix/security-vulnerabilities

# List remote branches that might be obsolete
Write-Host "The following remote branches may be obsolete:" -ForegroundColor Yellow
git for-each-ref --format='%(committerdate:short) %(refname:short)' --sort=committerdate refs/remotes/anya-core/

Write-Host "`nTo delete an obsolete remote branch, use:" -ForegroundColor Green
Write-Host "git push anya-core --delete BRANCH_NAME" -ForegroundColor Cyan

Write-Host "`nDon't forget to review stashed changes with 'git stash list' and apply or drop them as needed." -ForegroundColor Magenta
