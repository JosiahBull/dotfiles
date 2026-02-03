#!/bin/bash
# shellcheck disable=SC1091

set -o errexit -o pipefail -o noclobber

# ==============================================================================
# CONFIGURATION & CONSTANTS
# ==============================================================================

# Script directory (Portable realpath)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
HOME_DIR="$HOME"
LOCAL_BIN="$HOME_DIR/.local/bin"

# Colors for logging
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# User configuration (can be set via environment variables)
USER_NAME="${GIT_USER_NAME:-}"
USER_EMAIL="${GIT_USER_EMAIL:-}"

# ==============================================================================
# HELPER FUNCTIONS
# ==============================================================================

log() {
    echo -e "${BLUE}>>> ${NC}$1"
}

warn() {
    echo -e "${YELLOW}>>> WARNING: ${NC}$1"
}

error() {
    echo -e "${RED}>>> ERROR: ${NC}$1"
    exit 1
}

ensure_dir() {
    if [ ! -d "$1" ]; then
        mkdir -p "$1"
    fi
}

# ==============================================================================
# USER INFO COLLECTION
# ==============================================================================

collect_user_info() {
    # Skip in CI or non-interactive mode - use env variables or defaults
    if [ -n "$CI" ] || [ ! -t 0 ]; then
        if [ -z "$USER_NAME" ]; then
            USER_NAME="CI User"
        fi
        if [ -z "$USER_EMAIL" ]; then
            USER_EMAIL="ci@localhost"
        fi
        log "Using defaults (non-interactive): $USER_NAME <$USER_EMAIL>"
        return
    fi

    echo "============================================================="
    echo -e "${GREEN}User Configuration${NC}"
    echo "============================================================="
    echo "Please provide your details for Git and GPG configuration."
    echo "You can also set GIT_USER_NAME and GIT_USER_EMAIL environment"
    echo "variables to skip these prompts."
    echo "============================================================="
    echo ""

    # Collect name
    if [ -z "$USER_NAME" ]; then
        read -rp "Full name (for git commits): " USER_NAME
        if [ -z "$USER_NAME" ]; then
            error "Name is required."
        fi
    else
        log "Using name from environment: $USER_NAME"
    fi

    # Collect email
    if [ -z "$USER_EMAIL" ]; then
        read -rp "Email address (for git commits): " USER_EMAIL
        if [ -z "$USER_EMAIL" ]; then
            error "Email is required."
        fi
    else
        log "Using email from environment: $USER_EMAIL"
    fi

    echo ""
    log "Configuration: $USER_NAME <$USER_EMAIL>"
    echo ""
}

# ==============================================================================
# OS DETECTION & SYSTEM UPDATES
# ==============================================================================

detect_os() {
    case "$(uname -s)" in
        Linux*)     OS="Linux";;
        Darwin*)    OS="Mac";;
        *)          error "Unsupported OS: $(uname -s)" ;;
    esac
}

install_sys_packages() {
    log "Detected OS: $OS. Updating system and installing base dependencies..."

    if [ "$OS" = "Mac" ]; then
        if ! command -v brew &> /dev/null; then
            error "Homebrew is not installed. Please install it first (https://brew.sh/)."
        fi
        log "Updating Homebrew..."
        brew update && brew upgrade

        # Install Python, Pipx, and Utils
        brew install python pipx zsh tmux curl git gnupg pinentry-mac nano

        # Add pipx to path immediately for this session
        export PATH="$PATH:$HOME_DIR/Library/Python/3.9/bin"

    elif [ -f /etc/rocky-release ] || command -v dnf &> /dev/null; then
        log "Rocky/Fedora detected..."
        sudo dnf update -y

        if [ -f /etc/rocky-release ]; then
            sudo dnf install -y epel-release
            /usr/bin/crb enable
        fi

        sudo dnf install -y python3-pip pipx zsh tmux curl git gpg tar nano

    elif command -v apt-get &> /dev/null; then
        log "Debian/Ubuntu detected..."
        sudo apt-get update && sudo apt-get upgrade -y
        sudo apt-get install -y python3 python3-pip python3-venv pipx zsh tmux curl git gpg tar nano
    else
        error "No supported package manager found."
    fi

    # Ensure pipx path is set for the script execution
    ensure_dir "$LOCAL_BIN"
    export PATH="$LOCAL_BIN:$PATH"
    pipx ensurepath
}

# ==============================================================================
# PYTHON TOOLS (PIPX)
# ==============================================================================

install_python_tools() {
    log "Installing Python tools via pipx..."

    # Check if pipx works, if not, warn.
    if ! command -v pipx &> /dev/null; then
        warn "pipx not found in path. Attempting fallback or ensuring path."
    fi

    pipx install pre-commit
}

# ==============================================================================
# NODE.JS & TOOLS
# ==============================================================================

setup_node() {
    log "Setting up Node.js (nvm), pnpm, and Claude CLI..."

    export NVM_DIR="$HOME_DIR/.nvm"

    # 1. Install nvm if missing
    if [ ! -d "$NVM_DIR" ]; then
        log "Installing nvm..."
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
    else
        log "nvm already installed."
    fi

    # 2. Load nvm into current session
    # This allows us to use 'nvm', 'node', and 'npm' immediately below
    if [ -s "$NVM_DIR/nvm.sh" ]; then
        . "$NVM_DIR/nvm.sh"
    else
        warn "Could not load nvm.sh. Node installation may fail."
    fi

    # 3. Install/Update Node (LTS Version)
    log "Installing latest LTS Node.js..."
    nvm install --lts
    nvm use --lts

    # 4. Install Global Tools
    log "Installing global npm packages: pnpm..."
    npm install -g pnpm

    # 5. Install Claude Code (skip in CI - external dependency)
    if [ -n "$CI" ]; then
        log "Skipping Claude Code installation in CI environment."
    elif [ "$OS" = "Mac" ]; then
        log "Installing Claude Code via Homebrew Cask..."
        brew install --cask claude-code
    else
        log "Installing Claude Code via script (Linux fallback)..."
        curl -fsSL https://claude.ai/install.sh | bash
    fi
}

# ==============================================================================
# ZSH & DOTFILES
# ==============================================================================

setup_dotfiles() {
    log "Setting up dotfiles..."

    # Create completions directory (ZSH only)
    ensure_dir "$HOME_DIR/.zsh/completions"

    # Clone Scripts
    if [ -d "$HOME_DIR/.scripts" ]; then
        log "Scripts directory exists, pulling latest..."
        git -C "$HOME_DIR/.scripts" pull
    else
        git clone https://github.com/JosiahBull/shell-scripts "$HOME_DIR/.scripts"
    fi

    # Copy ZSH configs
    log "Copying ZSH configuration..."

    # We use -f to force overwrite, assuming the repo version is source of truth
    # Check if source files exist before copying
    if [ -f "$SCRIPT_DIR/zsh/.zshrc" ]; then
        cp "$SCRIPT_DIR/zsh/.zshrc" "$HOME_DIR/.zshrc"
        cp "$SCRIPT_DIR/zsh/.zsh_aliases" "$HOME_DIR/.zsh_aliases"
        cp "$SCRIPT_DIR/zsh/.p10k.zsh" "$HOME_DIR/.p10k.zsh"
    else
        warn "ZSH config files not found in $SCRIPT_DIR/zsh/. Skipping copy."
    fi

    # Handle Oh My Zsh
    if [ ! -d "$HOME_DIR/.oh-my-zsh" ]; then
        # If included in source, copy it, otherwise clone it
        if [ -d "$SCRIPT_DIR/zsh/ohmyzsh" ]; then
             cp -r "$SCRIPT_DIR/zsh/ohmyzsh" "$HOME_DIR/.oh-my-zsh"
        else
             log "Cloning Oh My Zsh..."
             git clone https://github.com/ohmyzsh/ohmyzsh.git "$HOME_DIR/.oh-my-zsh"
        fi
    fi

    # Install Plugins
    ZSH_CUSTOM="${ZSH_CUSTOM:-$HOME_DIR/.oh-my-zsh/custom}"

    log "Installing ZSH plugins/themes..."

    # Helper to clone only if not exists
    clone_plugin() {
        local url=$1
        local dest=$2
        if [ ! -d "$dest" ]; then
            git clone --depth=1 "$url" "$dest"
        else
            log "Plugin at $dest already exists."
        fi
    }

    clone_plugin "https://github.com/romkatv/powerlevel10k" "$ZSH_CUSTOM/themes/powerlevel10k"
    clone_plugin "https://github.com/zsh-users/zsh-autosuggestions" "$ZSH_CUSTOM/plugins/zsh-autosuggestions"
    clone_plugin "https://github.com/zsh-users/zsh-syntax-highlighting.git" "$ZSH_CUSTOM/plugins/zsh-syntax-highlighting"
}

# ==============================================================================
# GPG SETUP
# ==============================================================================

setup_gpg() {
    log "Configuring GPG..."
    ensure_dir "$HOME_DIR/.gnupg"
    chmod 700 "$HOME_DIR/.gnupg"

    if [ "$OS" = "Mac" ]; then
        log "Setting up GPG for macOS with pinentry-mac..."

        # Configure gpg-agent to use pinentry-mac
        PINENTRY_PATH="$(which pinentry-mac)"
        if [ -n "$PINENTRY_PATH" ]; then
            echo "pinentry-program $PINENTRY_PATH" > "$HOME_DIR/.gnupg/gpg-agent.conf"
            log "Configured pinentry-mac at: $PINENTRY_PATH"
        else
            warn "pinentry-mac not found. GPG signing may not work correctly."
        fi

        # Restart gpg-agent to pick up new config
        killall gpg-agent 2>/dev/null || true
    fi

    # Set correct permissions on gnupg directory
    chmod 600 "$HOME_DIR/.gnupg/"* 2>/dev/null || true

    # Skip interactive GPG key setup in CI or non-interactive mode
    if [ -n "$CI" ] || [ ! -t 0 ]; then
        log "Skipping GPG key generation (non-interactive mode)."
        return
    fi

    # Check if user already has a GPG key
    if gpg --list-secret-keys --keyid-format LONG 2>/dev/null | grep -q "sec"; then
        log "Existing GPG key found. Skipping key generation."
        return
    fi

    # Interactive GPG key generation
    echo "============================================================="
    echo -e "${GREEN}GPG Key Setup${NC}"
    echo "============================================================="
    echo "No GPG signing key found. To enable signed commits, you need to:"
    echo ""
    echo "  1. Generate a GPG key by running:"
    echo -e "     ${YELLOW}gpg --full-generate-key${NC}"
    echo ""
    echo "  2. Get your key ID by running:"
    echo -e "     ${YELLOW}gpg --list-secret-keys --keyid-format LONG${NC}"
    echo "     (Look for the ID after 'sec   ed25519/' or 'sec   rsa4096/')"
    echo ""
    echo "  3. Configure git to use your key:"
    echo -e "     ${YELLOW}git config --global user.signingkey <YOUR_KEY_ID>${NC}"
    echo ""
    echo "  4. Export your public key to add to GitHub:"
    echo -e "     ${YELLOW}gpg --armor --export <YOUR_KEY_ID> | pbcopy${NC}"
    echo ""
    echo "============================================================="
    echo ""
    read -p "Would you like to generate a GPG key now? [y/N] " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log "Generating GPG key for: $USER_NAME <$USER_EMAIL>"

        # Generate key using batch mode with collected user info
        gpg --batch --gen-key <<EOF
Key-Type: eddsa
Key-Curve: ed25519
Key-Usage: sign
Subkey-Type: ecdh
Subkey-Curve: cv25519
Subkey-Usage: encrypt
Name-Real: $USER_NAME
Name-Email: $USER_EMAIL
Expire-Date: 0
%commit
EOF

        echo ""
        log "Listing your GPG keys..."
        gpg --list-secret-keys --keyid-format LONG

        # Extract the key ID automatically
        GPG_KEY_ID=$(gpg --list-secret-keys --keyid-format LONG "$USER_EMAIL" 2>/dev/null | grep "sec" | head -1 | sed -E 's/.*\/([A-F0-9]+).*/\1/')

        if [ -n "$GPG_KEY_ID" ]; then
            git config --global user.signingkey "$GPG_KEY_ID"
            log "Configured git to use signing key: $GPG_KEY_ID"

            echo ""
            read -p "Copy public key to clipboard for GitHub? [y/N] " -n 1 -r
            echo ""
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                if [ "$OS" = "Mac" ]; then
                    gpg --armor --export "$GPG_KEY_ID" | pbcopy
                    log "Public key copied to clipboard. Add it to GitHub at: https://github.com/settings/keys"
                else
                    gpg --armor --export "$GPG_KEY_ID" | xclip -selection clipboard 2>/dev/null || \
                    gpg --armor --export "$GPG_KEY_ID" | xsel --clipboard 2>/dev/null || \
                    { log "Public key:"; gpg --armor --export "$GPG_KEY_ID"; }
                fi
            fi
        else
            warn "Could not automatically detect GPG key ID. Run 'gpg --list-secret-keys --keyid-format LONG' to find it."
        fi
    else
        log "Skipping GPG key generation. You can run 'gpg --full-generate-key' later."
    fi
}

# ==============================================================================
# SSH & GIT
# ==============================================================================

setup_ssh_git() {
    log "Configuring SSH and Git..."
    ensure_dir "$HOME_DIR/.ssh"

    # Copy base gitconfig
    if [ -f "$SCRIPT_DIR/.gitconfig" ]; then
        cp "$SCRIPT_DIR/.gitconfig" "$HOME_DIR/.gitconfig"
    fi

    # Set user info
    git config --global user.name "$USER_NAME"
    git config --global user.email "$USER_EMAIL"
    log "Set git user: $USER_NAME <$USER_EMAIL>"

    # Set GPG program path dynamically (works across all platforms)
    GPG_PATH="$(which gpg)"
    if [ -n "$GPG_PATH" ]; then
        git config --global gpg.program "$GPG_PATH"
        log "Set gpg.program to: $GPG_PATH"
    fi

    if [ -f "$SCRIPT_DIR/ssh_config" ]; then
        cp "$SCRIPT_DIR/ssh_config" "$HOME_DIR/.ssh/config"
    fi

    # Authorized keys from GitHub are handled by setup_ssh_key_sync() with deduplication

    # Generate Host Key if missing
    if [ ! -f "$HOME_DIR/.ssh/id_ed25519" ]; then
        log "Generating new SSH key..."
        ssh-keygen -t ed25519 -f "$HOME_DIR/.ssh/id_ed25519" -C "josiah@$(hostname)" -N ""
        eval "$(ssh-agent -s)"
        ssh-add "$HOME_DIR/.ssh/id_ed25519"
        cat "$HOME_DIR/.ssh/id_ed25519.pub" >> "$HOME_DIR/.ssh/authorized_keys"
    fi
}

# ==============================================================================
# SSH KEY SYNC CRON
# ==============================================================================

setup_ssh_key_sync() {
    log "Setting up SSH key sync cronjob..."

    # Install the sync script
    if [ -f "$SCRIPT_DIR/scripts/sync-ssh-keys.sh" ]; then
        cp "$SCRIPT_DIR/scripts/sync-ssh-keys.sh" "$LOCAL_BIN/sync-ssh-keys"
        chmod +x "$LOCAL_BIN/sync-ssh-keys"
        log "Installed sync-ssh-keys to $LOCAL_BIN/"
    else
        warn "sync-ssh-keys.sh not found in scripts/. Skipping SSH key sync setup."
        return
    fi

    # Ensure log directory exists
    ensure_dir "$HOME_DIR/.local/log"

    # Define cron job (every 6 hours)
    CRON_JOB="0 */6 * * * $LOCAL_BIN/sync-ssh-keys >> $HOME_DIR/.local/log/ssh-key-sync.log 2>&1"
    CRON_MARKER="# dotfiles-ssh-key-sync"

    # Skip cron setup in CI (no crontab available)
    if [ -n "$CI" ]; then
        log "Skipping cron setup (CI environment detected)."
    elif crontab -l 2>/dev/null | grep -q "$CRON_MARKER"; then
        log "SSH key sync cron job already installed."
    else
        # Add cron job
        (crontab -l 2>/dev/null || true; echo "$CRON_JOB $CRON_MARKER") | crontab -
        log "Installed SSH key sync cron job (runs every 6 hours)."
    fi

    # Run sync immediately (uses the new deduplicating script)
    log "Running initial SSH key sync..."
    "$LOCAL_BIN/sync-ssh-keys" || warn "Initial sync failed (network issue?)"
}

# ==============================================================================
# RUST TOOLS
# ==============================================================================

setup_rust() {
    log "Setting up Rust tooling..."

    ensure_dir "$LOCAL_BIN"

    # Install Rustup and Rust
    if ! command -v cargo &> /dev/null; then
        log "Installing Rust via rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        . "$HOME_DIR/.cargo/env"
    fi

    # Install stable toolchain with rust-src
    log "Installing stable toolchain with rust-src..."
    rustup toolchain install stable
    rustup default stable
    rustup component add rust-src

    # Install nightly toolchain with rust-src
    log "Installing nightly toolchain with rust-src..."
    rustup toolchain install nightly
    rustup component add rust-src --toolchain nightly

    # Install binstall
    if ! command -v cargo-binstall &> /dev/null; then
        curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    fi

    # Define tools: "PackageName:BinaryName"
    # If binary name is same as package, just put "PackageName"
    tools=(
        "bat"
        "cargo-autoinherit"
        "cargo-expand"
        "cargo-semver-checks"
        "cargo-tarpaulin"
        "cargo-udeps"
        "cargo-workspaces"
        "ripgrep:rg"
        "tokei"
        "cargo-mutants"
        "just"
        "cargo-deny"
        "cargo-insta"
        "cargo-release"
    )

    for item in "${tools[@]}"; do
        # Split by colon
        PACKAGE="${item%%:*}"
        BINARY="${item##*:}"

        log "Installing Rust tool: $PACKAGE..."

        # Install
        cargo binstall --no-confirm "$PACKAGE"

        # Move binary to local bin to survive rustup uninstall
        if [ -f "$HOME_DIR/.cargo/bin/$BINARY" ]; then
            mv -f "$HOME_DIR/.cargo/bin/$BINARY" "$LOCAL_BIN/$BINARY"
        else
            warn "Could not find installed binary for $PACKAGE at $HOME_DIR/.cargo/bin/$BINARY"
            continue
        fi

        # Generate completions (ZSH ONLY)
        # Note: We execute the moved binary from LOCAL_BIN
        local EXE="$LOCAL_BIN/$BINARY"

        case "$PACKAGE" in
            bat)
                "$EXE" --completion zsh > "$HOME_DIR/.zsh/completions/_bat"
                ;;
            ripgrep)
                "$EXE" --generate=complete-zsh > "$HOME_DIR/.zsh/completions/_rg"
                ;;
            cargo-mutants)
                "$EXE" mutants --completions zsh > "$HOME_DIR/.zsh/completions/_mutants"
                ;;
            just)
                "$EXE" --completions zsh > "$HOME_DIR/.zsh/completions/_just"
                ;;
        esac
    done

    # Generate rustup and cargo completions
    log "Generating rustup and cargo shell completions..."
    rustup completions zsh > "$HOME_DIR/.zsh/completions/_rustup"
    rustup completions zsh cargo > "$HOME_DIR/.zsh/completions/_cargo"

    # Clean up cargo-binstall (no longer needed after installs)
    cargo uninstall cargo-binstall || true

    log "Rust setup complete. Installed toolchains:"
    rustup show
}

# ==============================================================================
# MAIN EXECUTION
# ==============================================================================

main() {
    # Check args
    if [ "$#" -ne 0 ]; then
        echo "Usage: $0"
        exit 1
    fi

    # Sanity checks
    if [ ! -d "$HOME" ]; then
        error "\$HOME does not exist."
    fi

    log "Starting installation in tmpdir: $SCRIPT_DIR"

    detect_os
    collect_user_info
    install_sys_packages
    install_python_tools
    setup_node
    setup_dotfiles
    setup_gpg
    setup_ssh_git
    setup_ssh_key_sync
    setup_rust

    # Change Shell (skip in CI/non-interactive - requires PAM authentication)
    if [ -z "$CI" ] && [ -t 0 ] && [ "$SHELL" != "$(which zsh)" ]; then
        log "Changing shell to zsh..."
        chsh -s "$(which zsh)"
    elif [ -n "$CI" ] || [ ! -t 0 ]; then
        log "Skipping shell change (non-interactive mode)."
    fi

    log "Installation Complete! Please restart your terminal."
}

main "$@"
