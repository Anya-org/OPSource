# PowerShell script to identify and handle empty files
# This script will help identify and manage empty files in the repository

# Function to find empty files
function Find-EmptyFiles {
    Write-Host "Searching for empty files in the repository..." -ForegroundColor Yellow
    
    $emptyFiles = Get-ChildItem -Recurse -File | Where-Object { $_.Length -eq 0 }
    
    if ($emptyFiles.Count -eq 0) {
        Write-Host "No empty files found!" -ForegroundColor Green
        return @()
    }
    
    Write-Host "`nFound $($emptyFiles.Count) empty files:" -ForegroundColor Cyan
    
    $fileList = @()
    $i = 1
    foreach ($file in $emptyFiles) {
        $relativePath = $file.FullName.Replace((Get-Location).Path + "\", "")
        Write-Host "$i. $relativePath" -ForegroundColor White
        $fileList += $relativePath
        $i++
    }
    
    return $fileList
}

# Function to handle empty files
function Handle-EmptyFiles {
    param (
        [array]$Files
    )
    
    if ($Files.Count -eq 0) {
        return
    }
    
    Write-Host "`nOptions:" -ForegroundColor Magenta
    Write-Host "1: Open a file in default editor" -ForegroundColor Cyan
    Write-Host "2: Delete a file" -ForegroundColor Cyan
    Write-Host "3: Delete all empty files" -ForegroundColor Cyan
    Write-Host "4: Exit" -ForegroundColor Cyan
    
    $choice = Read-Host "Enter your choice (1-4)"
    
    switch ($choice) {
        1 {
            $fileNum = Read-Host "Enter file number to open (1-$($Files.Count))"
            if ([int]$fileNum -ge 1 -and [int]$fileNum -le $Files.Count) {
                Write-Host "Opening $($Files[$fileNum-1])..." -ForegroundColor Yellow
                start $Files[$fileNum-1]
            } else {
                Write-Host "Invalid file number." -ForegroundColor Red
            }
            Handle-EmptyFiles -Files $Files
        }
        2 {
            $fileNum = Read-Host "Enter file number to delete (1-$($Files.Count))"
            if ([int]$fileNum -ge 1 -and [int]$fileNum -le $Files.Count) {
                $fileToDelete = $Files[$fileNum-1]
                Write-Host "Deleting $fileToDelete..." -ForegroundColor Yellow
                Remove-Item $fileToDelete
                $Files = Find-EmptyFiles
                Handle-EmptyFiles -Files $Files
            } else {
                Write-Host "Invalid file number." -ForegroundColor Red
                Handle-EmptyFiles -Files $Files
            }
        }
        3 {
            $confirm = Read-Host "Are you sure you want to delete ALL empty files? (yes/no)"
            if ($confirm -eq "yes") {
                Write-Host "Deleting all empty files..." -ForegroundColor Yellow
                foreach ($file in $Files) {
                    Write-Host "Deleting $file..." -ForegroundColor Yellow
                    Remove-Item $file
                }
                Write-Host "All empty files deleted." -ForegroundColor Green
            } else {
                Write-Host "Operation cancelled." -ForegroundColor Red
                Handle-EmptyFiles -Files $Files
            }
        }
        4 {
            # Exit
            return
        }
        default {
            Write-Host "Invalid choice, please try again." -ForegroundColor Red
            Handle-EmptyFiles -Files $Files
        }
    }
}

# Main execution
Write-Host "===== Empty Files Manager =====" -ForegroundColor Magenta
$emptyFiles = Find-EmptyFiles
Handle-EmptyFiles -Files $emptyFiles

Write-Host "`nExiting Empty Files Manager. Goodbye!" -ForegroundColor Green
