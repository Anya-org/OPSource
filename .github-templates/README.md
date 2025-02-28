# Anya-org GitHub Templates

This directory contains standardized templates for all repositories in the Anya-org organization. These templates ensure consistency across all projects and align with Bitcoin principles of decentralization, security, privacy, and compatibility.

## Available Templates

| Template | Purpose |
|----------|---------|
| `.github-profile-readme.md` | Organization profile README for the main GitHub page |
| `CODE_OF_CONDUCT-template.md` | Code of conduct for all repositories |
| `CONTRIBUTING-template.md` | Guidelines for contributing to Anya projects |
| `PR-TEMPLATE.md` | Template for pull request submissions |
| `bug_report.md` | Template for bug report issues |
| `feature_request.md` | Template for feature request issues |
| `rust-ci-workflow.yml` | CI workflow for Rust projects |
| `security-scan-workflow.yml` | Security scanning workflow for dependencies |
| `MIT-LICENSE-template.txt` | MIT License template |

## How to Use These Templates

### For a New Repository

1. Create the repository structure:
   ```bash
   mkdir -p .github/ISSUE_TEMPLATE
   ```

2. Copy the templates to their respective locations:
   ```bash
   # For the repository root
   cp CONTRIBUTING-template.md ../CONTRIBUTING.md
   cp CODE_OF_CONDUCT-template.md ../CODE_OF_CONDUCT.md
   cp MIT-LICENSE-template.txt ../LICENSE

   # For pull request template
   cp PR-TEMPLATE.md ../.github/PULL_REQUEST_TEMPLATE.md

   # For issue templates
   cp bug_report.md ../.github/ISSUE_TEMPLATE/
   cp feature_request.md ../.github/ISSUE_TEMPLATE/

   # For workflows
   mkdir -p ../.github/workflows
   cp rust-ci-workflow.yml ../.github/workflows/rust.yml
   cp security-scan-workflow.yml ../.github/workflows/security-scan.yml
   ```

3. Customize the templates as needed, replacing placeholder values with actual project-specific information.

### For Existing Repositories

Same process as above, but be careful not to overwrite existing customizations. Consider running a diff first to see what changes would be made.

## Security Maintenance

The security scanner should be run regularly to ensure all dependencies are up-to-date and free from known vulnerabilities. It can be run manually with:

```pwsh
pwsh -File security-scan.ps1
```

This will help maintain the project's alignment with Bitcoin's core security principles.

## Branch Protection

After setting up templates, configure branch protection rules:

1. Go to repository Settings → Branches
2. Add rule for `main` branch
3. Require pull request reviews before merging
4. Require status checks to pass before merging
5. Include administrators to ensure everyone follows the process

## Template Updates

These templates should be reviewed quarterly to ensure they remain up-to-date with current best practices and Bitcoin ecosystem developments. When updating:

1. Make changes in this directory first
2. Test changes in a development repository
3. Roll out to all repositories in the organization

Following this process helps maintain consistency and quality across all Anya-org projects.
