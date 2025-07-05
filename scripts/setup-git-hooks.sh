#!/bin/sh

cp scripts/git-hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

echo "Installed pre-commit hook to .git/hooks/pre-commit" 