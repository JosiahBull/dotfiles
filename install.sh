#!/bin/bash
# shellcheck disable=SC1091

set -o errexit -o pipefail -o noclobber

DOTFILES_DIR="${DOTFILES_DIR:-$HOME/.dotfiles}"

if [ -d "$DOTFILES_DIR" ]; then
    echo "Updating existing dotfiles at $DOTFILES_DIR..."
    git -C "$DOTFILES_DIR" pull
    git -C "$DOTFILES_DIR" submodule update --init --recursive --depth 2
else
    echo "Cloning dotfiles to $DOTFILES_DIR..."
    git clone --recursive https://github.com/JosiahBull/dotfiles.git "$DOTFILES_DIR"
    git -C "$DOTFILES_DIR" submodule update --init --recursive --depth 2
fi

cd "$DOTFILES_DIR"
./configure.sh
