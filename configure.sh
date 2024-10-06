#!/bin/bash
# shellcheck disable=SC1091

set -o errexit -o pipefail -o noclobber

# Check that no args are passed
if [ "$#" -ne 0 ]; then
    echo ">>> Usage: $0"
    exit 1
fi

# Check that $HOME exists and exists and is a directory
if [ ! -d "$HOME" ]; then
    echo ">>> Error: \$HOME does not exist or is not a directory"
    exit 1
fi

# set tmpdir to the actual location of this script
tmpdir=$(dirname "$(realpath "$0")")

echo "tmpdir: $tmpdir"
echo "HOME: $HOME"

# check if dnf command exists
if command -v dnf &> /dev/null
then
    echo ">>> dnf found, updating"
    sudo dnf update -y

    # if rocky linux, install epel-release
    if [ -f /etc/rocky-release ]; then
        echo ">>> Rocky Linux detected, installing epel-release"
        sudo dnf install -y epel-release
        /usr/bin/crb enable
    fi

    # Install python utils.
    echo ">>> Installing python utils"
    sudo dnf install -y python3 python3-pip python3-venv

    # Install other applications.
    echo ">>> Installing other applications"
    sudo dnf install -y zsh tmux curl git gpg tar

    echo ">>> dnf complete"
# check if apt-get command exists
elif command -v apt-get &> /dev/null
then
    echo ">>> apt-get found, updating"
    sudo apt-get update && sudo apt-get upgrade -y

    # Install python utils.
    echo ">>> Installing python utils"
    sudo apt-get install -y python3 python3-pip python3-venv

    # Install other applications.
    echo ">>> Installing other utils"
    sudo apt-get install -y zsh tmux curl git gpg tar

    echo "apt-get complete"
else
    echo ">>> Could not install packages no package manager found"
    exit 1
fi

# install python modules
# I mostly only use `thefuck` for creating new git branches... eventually I'll replace it with a handful of shell scripts. :P
python3 -m venv ~/.local --system-site-packages
~/.local/bin/pip install thefuck pre-commit

# Move into the temporary directory.
pushd "$tmpdir"

# begin installation of dotfiles
echo ">>> Cloning scripts..."
git clone https://github.com/JosiahBull/shell-scripts "$HOME"/.scripts

# install relevant zsh plugins
echo ">>> Copying static files"
cp "$tmpdir/zsh/.zshrc" "$HOME/.zshrc"
cp "$tmpdir/zsh/.zsh_aliases" "$HOME/.zsh_aliases"
cp "$tmpdir/zsh/.p10k.zsh" "$HOME/.p10k.zsh"
cp -r "$tmpdir/zsh/ohmyzsh" "$HOME/.oh-my-zsh"

echo ">>> Cloning zsh plugins from github."
git clone --depth=1 https://gitee.com/romkatv/powerlevel10k.git "${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k"
git clone https://github.com/zsh-users/zsh-autosuggestions "$HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions"
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git "$HOME/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting"

# install ssh and git settings
echo ">>> Setting up ssh and git configuration"
mkdir -p "$HOME/.ssh"
cp "$tmpdir/.gitconfig" "$HOME/.gitconfig"
cp "$tmpdir/ssh_config" "$HOME/.ssh/config"
### TODO: We want to setup gpg keys here too!!

# copy ssh keys from https://github.com/josiahBull.keys to ~/.ssh/authorized_keys
# Just in case someone else is using this script, we'll print a large obvious warning with a delay
# so they can cancel the script if they don't want to add my keys to their authorized_keys file.
echo "============================================================="
echo "WARNING: Adding my SSH keys to your authorized_keys file."
echo "If you don't want to do this, press Ctrl+C now to cancel."
echo "============================================================="
sleep 20
curl https://github.com/josiahbull.keys >> ~/.ssh/authorized_keys

# Install Rust (to build tooling).
curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"

# Install various Rust tools. To save compute we'll prefer binaries and fall back to building from source.
# First, install cargo-binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Then, install the rest of the programs - ideally using a binary but we will fallback to building.
# XXX: Eventually we want to enable sign checking on packages here...
cargo binstall --no-confirm bat
cargo binstall --no-confirm cargo-autoinherit
cargo binstall --no-confirm cargo-expand
cargo binstall --no-confirm cargo-semver-checks
cargo binstall --no-confirm cargo-tarpaulin
cargo binstall --no-confirm cargo-udeps
cargo binstall --no-confirm cargo-update
cargo binstall --no-confirm cargo-workspaces
cargo binstall --no-confirm license-generator
cargo binstall --no-confirm ripgrep
cargo binstall --no-confirm tokei
cargo binstall --no-confirm zoxide
# XXX: Some of these should be installed with a wrapper script at first invocation/checks for updates
# after that... but that's a problem for future me.

# We remove Rust and cargo-binstall after we're done installing the tools and just keep the binaries.
# NOTE: Rustup toolchains consume 1.2GB of space in the image... which is frankly insane.
cargo uninstall cargo-binstall
rustup self uninstall -y

# create a new ed25519 keypair for this machine, if we are in a DE and a key doesn't exist already.
mkdir -p "$HOME/.ssh"
if [ ! -f "$HOME/.ssh/id_ed25519" ]; then
    echo "No SSH key found. Generating a new ed25519 keypair..."
    ssh-keygen -t ed25519 -f "$HOME/.ssh/id_ed25519" -C josiah
fi
if [ -z "$SSH_AGENT_PID" ]; then
    echo "No SSH agent found. Starting a new SSH agent..."
    eval "$(ssh-agent -s)"
fi
ssh-add "$HOME/.ssh/id_ed25519"
cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys

# chsh to zsh
chsh -s "$(which zsh)"
