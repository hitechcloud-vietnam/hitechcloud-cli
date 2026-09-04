#!/usr/bin/env bash
# Setup Git Hooks for HiTechCloud CLI
# Tự động cài đặt git hooks để đảm bảo commit luôn có Co-Authored-By

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"
GITHUB_HOOKS_DIR="$PROJECT_ROOT/.github/hooks"

echo "🔧 Setting up Git Hooks for HiTechCloud CLI..."

# Kiểm tra xem có phải git repository không
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "❌ Error: Not a git repository. Please run 'git init' first."
    exit 1
fi

# Tạo thư mục hooks nếu chưa có
mkdir -p "$HOOKS_DIR"

# Copy prepare-commit-msg hook
if [ -f "$GITHUB_HOOKS_DIR/prepare-commit-msg" ]; then
    cp "$GITHUB_HOOKS_DIR/prepare-commit-msg" "$HOOKS_DIR/prepare-commit-msg"
    chmod +x "$HOOKS_DIR/prepare-commit-msg"
    echo "✅ Installed prepare-commit-msg hook"
else
    echo "⚠️  Warning: prepare-commit-msg not found in .github/hooks/"
fi

# Cấu hình git commit template
if [ -f "$PROJECT_ROOT/.github/git-commit-template.txt" ]; then
    git config commit.template "$PROJECT_ROOT/.github/git-commit-template.txt"
    echo "✅ Configured git commit template"
fi

# Lấy và hiển thị GitHub user hiện tại
if command -v gh &> /dev/null; then
    GITHUB_USER=$(gh api user --jq '.login' 2>/dev/null)
    GITHUB_EMAIL=$(gh api user --jq '.email' 2>/dev/null)
    
    if [ -n "$GITHUB_USER" ]; then
        echo ""
        echo "👤 Current GitHub User: $GITHUB_USER"
        if [ -n "$GITHUB_EMAIL" ] && [ "$GITHUB_EMAIL" != "null" ]; then
            echo "📧 Email: $GITHUB_EMAIL"
        fi
        echo ""
        echo "📝 Git config updated:"
        echo "   user.name = $GITHUB_USER"
        echo "   user.email = $GITHUB_EMAIL"
        
        # Cấu hình git user
        git config user.name "$GITHUB_USER"
        if [ -n "$GITHUB_EMAIL" ] && [ "$GITHUB_EMAIL" != "null" ]; then
            git config user.email "$GITHUB_EMAIL"
        else
            git config user.email "${GITHUB_USER}@users.noreply.github.com"
        fi
    else
        echo "⚠️  Could not fetch GitHub user. Please run 'gh auth login' first."
    fi
else
    echo "⚠️  GitHub CLI (gh) not found. Install it from https://cli.github.com/"
    echo "   Using git config user.name/email as fallback."
fi

echo ""
echo "✨ Git hooks setup complete!"
echo ""
echo "📋 Commit format:"
echo "   <type>(<scope>): <description>"
echo ""
echo "   Co-Authored-By: $GITHUB_USER <${GITHUB_EMAIL:-$GITHUB_USER@users.noreply.github.com}>"
echo "   Co-Authored-By: HiTechCloud Agents <agent@hitechcloud.vn>"
echo ""
