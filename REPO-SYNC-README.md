# Anya-org Repository Synchronization Tools

These tools help maintain consistent standards across all repositories in the Anya-org GitHub organization. They ensure that all repositories follow the same documentation structure, CI/CD workflows, security checks, and contribution guidelines.

## Available Tools

1. **PowerShell Script** (`sync-repos.ps1`): For Windows users
2. **Bash Script** (`sync-repos.sh`): For Linux/macOS users

Both scripts perform the same function and will:

- Fetch all repositories in the Anya-org organization
- Clone each repository locally
- Create a new branch for template changes
- Apply standardized templates:
  - Contributing guidelines
  - Code of conduct
  - License
  - Pull request template
  - Issue templates
  - CI/CD workflows (for Rust projects)
  - Security scanning (for Rust projects)
- Commit and push changes
- Create pull requests (if GitHub token is provided)

## Prerequisites

- Git installed and configured
- PowerShell (for Windows) or Bash (for Linux/macOS)
- GitHub personal access token (for private repositories and creating PRs)
- Internet connection to access GitHub API

## Usage

### PowerShell Script (Windows)

```powershell
# Dry run (no changes will be made)
.\sync-repos.ps1 -DryRun

# With GitHub token provided inline
.\sync-repos.ps1 -GithubToken "your_github_token"

# Without token (will prompt or use limited public API)
.\sync-repos.ps1
```

### Bash Script (Linux/macOS)

```bash
# Make the script executable
chmod +x sync-repos.sh

# Dry run (no changes will be made)
./sync-repos.sh --dry-run

# With GitHub token provided inline
./sync-repos.sh --token "your_github_token"

# Without token (will prompt or use limited public API)
./sync-repos.sh
```

## GitHub Token

A GitHub personal access token is recommended for:
- Accessing private repositories
- Avoiding rate limits
- Creating pull requests automatically

To create a token:
1. Go to GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate a new token with the following permissions:
   - `repo` (Full control of private repositories)
   - `workflow` (if you need to update GitHub Actions workflows)
3. Copy the token and use it with the script

## Customizing the Sync Process

If you need to customize the synchronization process:

1. Edit the script to modify which files are copied
2. Adjust the template files in the `.github-templates` directory
3. Modify the commit message or PR description as needed

## Maintaining Different Repository Types

For repositories with different requirements:

- The script automatically detects Rust projects by checking for `Cargo.toml`
- You can extend the scripts to detect other project types (e.g., JavaScript, Python)
- Add conditions to apply different templates based on repository type

## Alignment with Bitcoin Principles

These synchronization tools support Bitcoin principles by:

1. **Decentralization**: Ensuring each repository can operate independently while maintaining standards
2. **Security**: Implementing security scanning and best practices across all codebases
3. **Privacy**: Applying consistent privacy-focused coding standards
4. **Compatibility**: Maintaining consistent interfaces and documentation structure

## Troubleshooting

If you encounter issues:

- **Rate limits**: Use a GitHub token to avoid API rate limits
- **Authentication failures**: Ensure your GitHub token has sufficient permissions
- **Failed clones**: Check repository accessibility and network connection
- **Merge conflicts**: The script creates PRs which may need manual resolution if conflicts exist

## Best Practices

- Run with `-DryRun` or `--dry-run` first to verify expected changes
- Review generated PRs before merging
- Update templates in the `.github-templates` directory first, then sync repositories
- Run periodically to ensure all repositories remain in sync
