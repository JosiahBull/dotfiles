#!/bin/bash
# ==============================================================================
# SSH Key Sync Script
# Fetches SSH public keys from GitHub and syncs to authorized_keys
# ==============================================================================

set -o errexit -o pipefail

# Configuration
GITHUB_USER="${GITHUB_SSH_USER:-josiahbull}"
AUTHORIZED_KEYS="$HOME/.ssh/authorized_keys"
TEMP_FILE=$(mktemp)
LOCK_FILE="/tmp/sync-ssh-keys.lock"

# Logging
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

error() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ERROR: $1" >&2
    exit 1
}

# Cleanup on exit
cleanup() {
    rm -f "$TEMP_FILE" "${TEMP_FILE}.merged"
    rm -f "$LOCK_FILE"
}
trap cleanup EXIT

# Prevent concurrent runs
if [ -f "$LOCK_FILE" ]; then
    log "Another instance is running (lock file exists). Exiting."
    exit 0
fi
touch "$LOCK_FILE"

# Ensure .ssh directory exists with correct permissions
if [ ! -d "$HOME/.ssh" ]; then
    mkdir -p "$HOME/.ssh"
    chmod 700 "$HOME/.ssh"
fi

# Fetch keys from GitHub
log "Fetching SSH keys for $GITHUB_USER from GitHub..."
if ! curl -sf "https://github.com/${GITHUB_USER}.keys" > "$TEMP_FILE"; then
    error "Failed to fetch keys from GitHub. Network issue or user not found."
fi

# Validate fetched content (should contain ssh- prefix)
if ! grep -q "^ssh-" "$TEMP_FILE"; then
    error "Fetched content does not appear to contain valid SSH keys."
fi

FETCHED_COUNT=$(wc -l < "$TEMP_FILE" | tr -d ' ')
log "Fetched $FETCHED_COUNT key(s) from GitHub."

# Create authorized_keys if it doesn't exist
if [ ! -f "$AUTHORIZED_KEYS" ]; then
    touch "$AUTHORIZED_KEYS"
    chmod 600 "$AUTHORIZED_KEYS"
fi

# Merge and deduplicate
{
    cat "$AUTHORIZED_KEYS" 2>/dev/null || true
    cat "$TEMP_FILE"
} | sort -u > "${TEMP_FILE}.merged"

# Count changes
BEFORE_COUNT=$(wc -l < "$AUTHORIZED_KEYS" 2>/dev/null | tr -d ' ' || echo "0")
AFTER_COUNT=$(wc -l < "${TEMP_FILE}.merged" | tr -d ' ')

# Only update if content differs
if ! diff -q "$AUTHORIZED_KEYS" "${TEMP_FILE}.merged" > /dev/null 2>&1; then
    cp "${TEMP_FILE}.merged" "$AUTHORIZED_KEYS"
    chmod 600 "$AUTHORIZED_KEYS"
    ADDED=$((AFTER_COUNT - BEFORE_COUNT))
    log "Updated authorized_keys. Added $ADDED new key(s). Total: $AFTER_COUNT"
else
    log "No changes needed. All $FETCHED_COUNT GitHub key(s) already present."
fi

log "Sync complete."
