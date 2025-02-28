#!/bin/bash
# Repository Sync Script for Anya-org
# Follows Bitcoin principles of decentralization, security, and transparency

set -e

# Colors for output
COLOR_INFO='\033[0;36m'  # Cyan
COLOR_SUCCESS='\033[0;32m'  # Green
COLOR_WARNING='\033[0;33m'  # Yellow
COLOR_ERROR='\033[0;31m'  # Red
COLOR_RESET='\033[0m'

# Configuration
ORG_NAME="Anya-org"
TEMPLATE_DIR="$(pwd)/.github-templates"
GITHUB_TOKEN=""
DRY_RUN=false

function print_title() {
    echo -e "\n${COLOR_INFO}==== $1 ====${COLOR_RESET}"
}

function print_step() {
    echo -e "${COLOR_INFO}→ $1${COLOR_RESET}"
}

function print_success() {
    echo -e "${COLOR_SUCCESS}✓ $1${COLOR_RESET}"
}

function print_warning() {
    echo -e "${COLOR_WARNING}⚠ $1${COLOR_RESET}"
}

function print_error() {
    echo -e "${COLOR_ERROR}✗ $1${COLOR_RESET}"
}

function get_repositories() {
    print_step "Fetching repositories for $ORG_NAME..."
    
    if [ -z "$GITHUB_TOKEN" ]; then
        print_warning "No GitHub token provided. Using public API with rate limits."
        headers=""
    else
        headers="Authorization: token $GITHUB_TOKEN"
    fi
    
    page=1
    all_repos=()
    
    while true; do
        api_url="https://api.github.com/orgs/$ORG_NAME/repos?per_page=100&page=$page"
        
        if [ -z "$headers" ]; then
            response=$(curl -s "$api_url")
        else
            response=$(curl -s -H "$headers" "$api_url")
        fi
        
        # Check if response is empty or has no repos
        if [ "$response" = "[]" ]; then
            break
        fi
        
        # Extract repo names
        repos=$(echo "$response" | grep -o '"name":"[^"]*"' | sed 's/"name":"//g' | sed 's/"//g')
        
        if [ -z "$repos" ]; then
            break
        fi
        
        while IFS= read -r repo; do
            all_repos+=("$repo")
        done <<< "$repos"
        
        page=$((page + 1))
    done
    
    print_success "Found ${#all_repos[@]} repositories"
    echo "${all_repos[@]}"
}

function sync_repository() {
    repo_name="$1"
    repo_url="https://github.com/$ORG_NAME/$repo_name.git"
    temp_dir="/tmp/anya-sync-$repo_name"
    
    print_title "Syncing repository: $repo_name"
    
    if [ -d "$temp_dir" ]; then
        rm -rf "$temp_dir"
    fi
    
    # Clone the repository
    print_step "Cloning $repo_url to $temp_dir..."
    if [ "$DRY_RUN" = false ]; then
        if ! git clone "$repo_url" "$temp_dir"; then
            print_error "Failed to clone repository"
            return 1
        fi
    fi
    
    # Create a new branch
    branch_name="sync/github-templates-$(date +%Y%m%d)"
    print_step "Creating branch: $branch_name..."
    if [ "$DRY_RUN" = false ]; then
        cd "$temp_dir"
        default_branch=$(git symbolic-ref --short HEAD)
        
        if ! git checkout -b "$branch_name"; then
            print_error "Failed to create branch"
            cd - > /dev/null
            return 1
        fi
    fi
    
    # Create necessary directories
    print_step "Creating directories..."
    if [ "$DRY_RUN" = false ]; then
        github_dir="$temp_dir/.github"
        issue_template_dir="$github_dir/ISSUE_TEMPLATE"
        workflows_dir="$github_dir/workflows"
        
        mkdir -p "$issue_template_dir"
        mkdir -p "$workflows_dir"
    fi
    
    # Copy templates
    print_step "Copying templates..."
    if [ "$DRY_RUN" = false ]; then
        # Root files
        cp "$TEMPLATE_DIR/CONTRIBUTING-template.md" "$temp_dir/CONTRIBUTING.md"
        cp "$TEMPLATE_DIR/CODE_OF_CONDUCT-template.md" "$temp_dir/CODE_OF_CONDUCT.md"
        cp "$TEMPLATE_DIR/MIT-LICENSE-template.txt" "$temp_dir/LICENSE"
        
        # GitHub directory
        cp "$TEMPLATE_DIR/PR-TEMPLATE.md" "$github_dir/PULL_REQUEST_TEMPLATE.md"
        
        # Issue templates
        cp "$TEMPLATE_DIR/bug_report.md" "$issue_template_dir/bug_report.md"
        cp "$TEMPLATE_DIR/feature_request.md" "$issue_template_dir/feature_request.md"
        
        # Workflows for Rust projects
        if [ -f "$temp_dir/Cargo.toml" ]; then
            print_step "Detected Rust project, adding Rust-specific workflows..."
            cp "$TEMPLATE_DIR/rust-ci-workflow.yml" "$workflows_dir/rust.yml"
            cp "$TEMPLATE_DIR/security-scan-workflow.yml" "$workflows_dir/security-scan.yml"
            cp "$(dirname $TEMPLATE_DIR)/security-scan.ps1" "$temp_dir/security-scan.ps1"
        fi
    fi
    
    # Commit changes
    print_step "Committing changes..."
    if [ "$DRY_RUN" = false ]; then
        git add .
        if ! git commit -m "chore: Sync GitHub templates and standards

Apply organization-wide templates:
- Add contributing guidelines
- Add code of conduct
- Add PR template
- Add issue templates
- Add workflows for CI/CD
- Add security scan

This ensures consistency across all Anya-org repositories and
aligns with our Bitcoin principles of decentralization,
security, privacy, and compatibility."; then
            print_warning "No changes to commit or commit failed"
            cd - > /dev/null
            return 1
        fi
    fi
    
    # Push changes and create PR
    print_step "Pushing changes and creating PR..."
    if [ "$DRY_RUN" = false ]; then
        if [ -n "$GITHUB_TOKEN" ]; then
            # Configure git with token for push
            repo_url_with_token=${repo_url/https:\/\//https:\/\/$GITHUB_TOKEN@}
            if ! git push -u "$repo_url_with_token" "$branch_name"; then
                print_error "Failed to push changes"
                cd - > /dev/null
                return 1
            fi
            
            # Create PR using GitHub API
            pr_url="https://api.github.com/repos/$ORG_NAME/$repo_name/pulls"
            pr_body='{
                "title": "Sync GitHub templates and standards",
                "body": "This PR syncs the repository with organization-wide templates and standards:\n\n## Changes included:\n- Add contributing guidelines\n- Add code of conduct\n- Add PR template\n- Add issue templates\n- Add workflows for CI/CD (where applicable)\n- Add security scan (for Rust projects)\n\nThis ensures consistency across all Anya-org repositories and aligns with our Bitcoin principles of decentralization, security, privacy, and compatibility.\n\n**Note:** Please review and adjust any repository-specific configurations as needed.",
                "head": "'"$branch_name"'",
                "base": "'"$default_branch"'"
            }'
            
            pr_response=$(curl -s -X POST -H "Authorization: token $GITHUB_TOKEN" -H "Accept: application/vnd.github.v3+json" -d "$pr_body" "$pr_url")
            pr_url=$(echo "$pr_response" | grep -o '"html_url":"[^"]*"' | head -1 | sed 's/"html_url":"//g' | sed 's/"//g')
            
            if [ -n "$pr_url" ]; then
                print_success "Created PR: $pr_url"
            else
                print_error "Failed to create PR"
            fi
        else
            if ! git push -u origin "$branch_name"; then
                print_error "Failed to push changes"
                cd - > /dev/null
                return 1
            fi
            print_warning "GitHub token not provided. Please create PR manually."
        fi
        
        cd - > /dev/null
    fi
    
    print_success "Repository sync complete for $repo_name"
    return 0
}

function sync_all_repositories() {
    print_title "Starting repository sync process"
    
    if [ "$DRY_RUN" = true ]; then
        print_warning "Running in DRY-RUN mode. No changes will be made."
    fi
    
    repos=($(get_repositories))
    success_count=0
    fail_count=0
    
    for repo in "${repos[@]}"; do
        if sync_repository "$repo"; then
            success_count=$((success_count + 1))
        else
            fail_count=$((fail_count + 1))
        fi
    done
    
    print_title "Sync Summary"
    echo -e "${COLOR_INFO}Total repositories: ${#repos[@]}${COLOR_RESET}"
    echo -e "${COLOR_SUCCESS}Successfully synced: $success_count${COLOR_RESET}"
    echo -e "${COLOR_ERROR}Failed to sync: $fail_count${COLOR_RESET}"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --token)
        GITHUB_TOKEN="$2"
        shift
        shift
        ;;
        --dry-run)
        DRY_RUN=true
        shift
        ;;
        *)
        echo "Unknown option: $key"
        exit 1
        ;;
    esac
done

# Main script execution
print_title "Anya-org Repository Sync"
echo "This script will sync all repositories in the Anya-org organization with standardized templates."

if [ ! -d "$TEMPLATE_DIR" ]; then
    print_error "Template directory not found at $TEMPLATE_DIR"
    exit 1
fi

# Prompt for GitHub token if not provided
if [ -z "$GITHUB_TOKEN" ]; then
    echo -n "Enter GitHub token (leave blank to continue without token): "
    read -r GITHUB_TOKEN
fi

sync_all_repositories

print_title "Sync Complete"
if [ "$DRY_RUN" = true ]; then
    print_warning "This was a dry run. No changes were made."
    echo "To perform actual changes, run the script without the --dry-run flag."
fi
