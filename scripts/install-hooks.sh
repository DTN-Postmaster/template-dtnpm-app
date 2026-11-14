#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
HOOK_PATH=".git/hooks/pre-commit"

mkdir -p .git/hooks
cat > "$HOOK_PATH" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "[pre-commit] Running strict security and quality gate..."
exec ./scripts/check-security.sh --strict
EOF
chmod +x "$HOOK_PATH"
echo "Installed pre-commit hook at $(pwd)/$HOOK_PATH"
