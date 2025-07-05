#!/bin/sh

echo "Setting up git hooks for cmdr..."
echo ""
echo "Choose your pre-commit hook style:"
echo "1) Check-only (recommended) - validates formatting without changing files"
echo "2) Auto-format - automatically formats code before commit"
echo ""
read -p "Enter your choice (1 or 2): " choice

case $choice in
  1)
    cp scripts/git-hooks/pre-commit .git/hooks/pre-commit
    echo "Installed check-only pre-commit hook"
    ;;
  2)
    cp scripts/git-hooks/pre-commit-auto-format .git/hooks/pre-commit
    echo "Installed auto-format pre-commit hook"
    ;;
  *)
    echo "Invalid choice. Installing check-only hook by default."
    cp scripts/git-hooks/pre-commit .git/hooks/pre-commit
    ;;
esac

chmod +x .git/hooks/pre-commit
echo "Pre-commit hook installed successfully!"
echo ""
echo "Note: The hook will run cargo fmt --check, cargo check, and cargo clippy before each commit."
echo "If any check fails, the commit will be aborted." 