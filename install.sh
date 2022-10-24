#!/bin/bash

# create a temporary directory to work in
tmpdir=`mktemp -d`

# check if dnf command exists
if command -v dnf &> /dev/null
then
    sudo dnf install -y zsh vim tmux curl neovim thefuck git gpg python3 util-linux-user openssh-askpass
    echo "dnf complete"
# check if apt command exists
elif command -v apt &> /dev/null
then
    sudo apt install -y zsh vim tmux curl neovim thefuck git gpg python3 ssh-askpass build-essential zsh-syntax-highlighting zsh-autosuggestions
    echo "apt complete"
else
    echo "Could not install packages no package manager found"
    exit 1
fi

# clone the current repository into a temporary directory, recursively with submodules
git clone https://github.com/JosiahBull/dotfiles $tmpdir
cd $tmpdir
git submodule update --init --recursive --depth 2

# begin installation of dotfiles

# copy .scripts folder to $HOME/.scripts
cp -r $tmpdir/.scripts $HOME/.scripts

# install zsh, thefuck, tmux, vim, curl, and git
# if on RHEL based system, fedora, rocky linux, or centos install with dnf
# if on debian based system, ubuntu, or mint install with apt

# install relevant zsh plugins
cp $tmpdir/zsh/.zshrc $HOME/.zshrc
cp $tmpdir/zsh/.zsh_aliases $HOME/.zsh_aliases
cp $tmpdir/zsh/.p10k.zsh $HOME/.p10k.zsh
cp -r $tmpdir/zsh/ohmyzsh $HOME/.oh-my-zsh

git clone --depth=1 https://gitee.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

# install ssh and git settings
cp $tmpdir/.gitconfig $HOME/.gitconfig

# copy ssh keys from https://github.com/josiahBull.keys to ~/.ssh/authorized_keys
curl https://github.com/josiahbull.keys >> ~/.ssh/authorized_keys

# add a cronjob to update the ssh keys every 48 hours
# echo "0 */48 * * * curl https://github.com/josiahbull.keys > ~/.ssh/authorized_keys" | crontab -

# Install NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
source "$HOME/.nvm/nvm.sh"

nvm install 'lts/*' --reinstall-packages-from=current

# chsh to zsh
sudo chsh $USER -s $(which zsh)

# install rust
curl https://sh.rustup.rs -sSf | sh -s -- -y

# install dds
~/.cargo/bin/cargo install dds --git https://github.com/josiahbull/dds/

# create a new ed25519 keypair for this machine
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -C josiah

# print the ed25519 public key and tell user to add it to the git server
echo "Add the following public key to the github/gitlab/gitmedia repositories"
cat ~/.ssh/id_ed25519.pub

# clean up the temporary directory
rm -rf $tmpdir
