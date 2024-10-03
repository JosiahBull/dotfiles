#!/bin/bash
# shellcheck disable=SC1091

set -o errexit -o pipefail -o noclobber

# create a temporary directory to work in
tmpdir=$(mktemp -d)

# Check if root, throw error if not.
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

# check if dnf command exists
if command -v dnf &> /dev/null
then
    dnf update -y

    # if rocky linux, install epel-release
    if [ -f /etc/rocky-release ]; then
        dnf install -y epel-release
        /usr/bin/crb enable
    fi

    dnf install -y zsh vim tmux curl neovim git gpg python3 util-linux-user openssh-askpass python3-pip gcc cmake tar firefox golang

    # Check if DESKTOP_SESSION is set, and install vscode if so.
    if [ -n "$DESKTOP_SESSION" ]; then
        rpm --import https://packages.microsoft.com/keys/microsoft.asc
        echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo > /dev/null
        dnf check-update
        dnf install code
    fi

    echo "dnf complete"
# check if apt command exists
elif command -v apt &> /dev/null
then
    apt-get update && apt-get upgrade -y

    # if debian 11, install firefox-esr otherwise install firefox
    if [ -f /etc/debian_version ]; then
        if [ "$(cut -d '.' -f 1 /etc/debian_version)" == "11" ]; then
            apt install -y firefox-esr
        else
            apt install -y firefox
        fi
    fi

    apt-get install -y zsh vim tmux curl neovim git gpg python3 ssh-askpass build-essential python3-pip gcc cmake tar golang apt-transport-https

    # Check if DESKTOP_SESSION is set, and install vscode if so.
    if [ -n "$DESKTOP_SESSION" ]; then
        curl -sSL https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
        install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg
        echo "deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" |sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null
        rm -f packages.microsoft.gpg
        apt-get install code
    fi

    echo "apt complete"
else
    echo "Could not install packages no package manager found"
    exit 1
fi

# install python modules
pip3 install thefuck pre-commit

# clone the current repository into a temporary directory, recursively with submodules
git clone https://github.com/JosiahBull/dotfiles "$tmpdir"
cd "$tmpdir" || exit
git submodule update --init --recursive --depth 2

# begin installation of dotfiles

# clone https://github.com/JosiahBull/shell-scripts to $HOME/.scripts
git clone https://github.com/JosiahBull/shell-scripts "$HOME"/.scripts

# install relevant zsh plugins
cp "$tmpdir/zsh/.zshrc" "$HOME/.zshrc"
cp "$tmpdir/zsh/.zsh_aliases" "$HOME/.zsh_aliases"
cp "$tmpdir/zsh/.p10k.zsh" "$HOME/.p10k.zsh"
cp -r "$tmpdir/zsh/ohmyzsh" "$HOME/.oh-my-zsh"

git clone --depth=1 https://gitee.com/romkatv/powerlevel10k.git "${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k"
git clone https://github.com/zsh-users/zsh-autosuggestions ~/.oh-my-zsh/custom/plugins/zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting

# install ssh and git settings
cp "$tmpdir/.gitconfig" "$HOME/.gitconfig"
cp "$tmpdir/ssh_config" "$HOME/.ssh/config"

# copy ssh keys from https://github.com/josiahBull.keys to ~/.ssh/authorized_keys
curl https://github.com/josiahbull.keys >> ~/.ssh/authorized_keys

# Install NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
source "$HOME/.nvm/nvm.sh"
nvm install 'lts/*' --reinstall-packages-from=current

# Install Yarn
curl -o- -L https://yarnpkg.com/install.sh | bash

# Add to .zshrc
{
    echo "export NVM_DIR=\"$HOME/.nvm\""
    echo "[ -s \"\$NVM_DIR/nvm.sh\" ] && \. \"\$NVM_DIR/nvm.sh\""
    echo "[ -s \"\$NVM_DIR/bash_completion\" ] && \. \"\$NVM_DIR/bash_completion\""
    echo "export PATH=\"\$HOME/.yarn/bin:\$HOME/.config/yarn/global/node_modules/.bin:\$PATH\""
} >> "$HOME/.zshrc"

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y

# Install rust programs from source
cargo install bat
cargo install ripgrep
cargo install cargo-workspaces
cargo install cargo-tarpaulin
cargo install cargo-udeps
cargo install tokei
cargo install cargo-expand
cargo install license-generator

# chsh to zsh
chsh "$USER" -s "$(which zsh)"

# create a new ed25519 keypair for this machine
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -C josiah

# add the new key to the ssh agent
ssh-add ~/.ssh/id_ed25519

# add our own key to authorized_keys
cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys

# grab all keys from https://github.com/JosiahBull.keys and add them to authorized_keys
curl https://github.com/JosiahBull.keys >> ~/.ssh/authorized_keys

# clean up the temporary directory
rm -rdf "$tmpdir"
