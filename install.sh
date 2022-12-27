#!/bin/bash

# read if the --reduced flag is set
reduced=false
if [[ $1 == "--reduced" ]]; then
    reduced=true
fi

# create a temporary directory to work in
tmpdir=`mktemp -d`

# check if dnf command exists
if command -v dnf &> /dev/null
then
    sudo dnf update -y

    # if rocky linux, install epel-release
    if [ -f /etc/rocky-release ]; then
        sudo dnf install -y epel-release
        sudo /usr/bin/crb enable
    fi

    sudo dnf install -y zsh vim tmux curl neovim git gpg python3 util-linux-user openssh-askpass python3-pip gcc cmake tar firefox
    echo "dnf complete"
# check if apt command exists
elif command -v apt &> /dev/null
then
    sudo apt-get update && sudo apt-get upgrade -y

    # if debian 11, install firefox-esr otherwise install firefox
    if [ -f /etc/debian_version ]; then
        if [ `cat /etc/debian_version | cut -d '.' -f 1` == "11" ]; then
            sudo apt install -y firefox-esr
        else
            sudo apt install -y firefox
        fi
    fi

    sudo apt install -y zsh vim tmux curl neovim git gpg python3 ssh-askpass build-essential python3-pip gcc cmake tar
    echo "apt complete"
else
    echo "Could not install packages no package manager found"
    exit 1
fi

# if previous commands weren't successful, exit
if [ $? -ne 0 ]; then
    echo "Could not install packages"
    exit 1
fi

# install python modules
sudo pip3 install thefuck

# if previous commands weren't successful, exit
if [ $? -ne 0 ]; then
    echo "Could not install python modules"
    exit 1
fi

# clone the current repository into a temporary directory, recursively with submodules
git clone https://github.com/JosiahBull/dotfiles $tmpdir
cd $tmpdir
git submodule update --init --recursive --depth 2

# if previous commands weren't successful, exit
if [ $? -ne 0 ]; then
    echo "Could not clone repository"
    exit 1
fi

# begin installation of dotfiles

# copy .scripts folder to $HOME/.scripts
cp -r $tmpdir/.scripts $HOME/.scripts

# install relevant zsh plugins
cp $tmpdir/zsh/.zshrc $HOME/.zshrc
cp $tmpdir/zsh/.zsh_aliases $HOME/.zsh_aliases
cp $tmpdir/zsh/.p10k.zsh $HOME/.p10k.zsh
cp -r $tmpdir/zsh/ohmyzsh $HOME/.oh-my-zsh

git clone --depth=1 https://gitee.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
git clone https://github.com/zsh-users/zsh-autosuggestions ~/.oh-my-zsh/custom/plugins/zsh-autosuggestions
git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ~/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting

# install ssh and git settings
cp $tmpdir/.gitconfig $HOME/.gitconfig
cp $tmpdir/ssh_config $HOME/.ssh/config

# copy ssh keys from https://github.com/josiahBull.keys to ~/.ssh/authorized_keys
curl https://github.com/josiahbull.keys >> ~/.ssh/authorized_keys

# if gnome is installed, (check if /usr/bin/gnome-session exists)
if [ -f /usr/bin/gnome-session ]; then
    # super + c -> open calculator
    python3 $tmpdir/add_shortcut.py "calculator" "gnome-calculator" "<Super>c"
    # ctrl + alt + t -> open terminal (if not already set)
    python3 $tmpdir/add_shortcut.py "terminal" "gnome-terminal" "<Primary><Alt>t"
    # super + t -> open terminal (if not already set)
    python3 $tmpdir/add_shortcut.py "terminal" "gnome-terminal" "<Super>t"
    # super + l -> lock screen
    python3 $tmpdir/add_shortcut.py "lock" "gnome-screensaver-command -l" "<Super>l"
    # super + e -> open the default file manager (nautlius, dolphin, etc.)
    python3 $tmpdir/add_shortcut.py "file manager" "xdg-open $HOME" "<Super>e"

    # ctrl + shift + up arrow = volume up
    python3 $tmpdir/add_shortcut.py "volume up" "amixer -q set Master 5%+" "<Primary><Shift>Up"
    # ctrl + shift + down arrow = volume down
    python3 $tmpdir/add_shortcut.py "volume down" "amixer -q set Master 5%-" "<Primary><Shift>Down"
    # ctrl + left = previous song
    python3 $tmpdir/add_shortcut.py "previous song" "playerctl previous" "<Primary>Left"
    # ctrl + right = next song
    python3 $tmpdir/add_shortcut.py "next song" "playerctl next" "<Primary>Right"
    # ctrl + down = pause/play
    python3 $tmpdir/add_shortcut.py "pause/play" "playerctl play-pause" "<Primary>Down"

    # install gnome extensions if the reduced flag isn't set
    if [ $reduced == false ]; then
        # save the current directory
        old_dir=`pwd`

        # install hide top bar extension
        cd $HOME/.local/share/gnome-shell/extensions/
        git clone https://github.com/tuxor1337/hidetopbar.git hidetopbar@mathieu.bidon.ca
        cd hidetopbar@mathieu.bidon.ca
        make
        cd ..
        gnome-extensions enable hidetopbar@mathieu.bidon.ca

        cd $old_dir
    fi
fi

######## START OPTIONAL INSTALL ########
if [ $reduced = false ]; then
    # Install NVM
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
    source "$HOME/.nvm/nvm.sh"
    nvm install 'lts/*' --reinstall-packages-from=current

    # Install Yarn
    curl -o- -L https://yarnpkg.com/install.sh | bash

    # Add to .zshrc
    echo "export NVM_DIR=\"$HOME/.nvm\"" >> $HOME/.zshrc
    echo "[ -s \"\$NVM_DIR/nvm.sh\" ] && \. \"\$NVM_DIR/nvm.sh\"" >> $HOME/.zshrc
    echo "[ -s \"\$NVM_DIR/bash_completion\" ] && \. \"\$NVM_DIR/bash_completion\"" >> $HOME/.zshrc
    echo "export PATH=\"\$HOME/.yarn/bin:\$HOME/.config/yarn/global/node_modules/.bin:\$PATH\"" >> $HOME/.zshrc

    # install rust
    curl https://sh.rustup.rs -sSf | sh -s -- -y
fi
######## END OPTIONAL INSTALL ########

# chsh to zsh
sudo chsh $USER -s $(which zsh)

# create a new ed25519 keypair for this machine
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -C josiah

# add the new key to the ssh agent
ssh-add ~/.ssh/id_ed25519

# add our own key to authorized_keys
cat ~/.ssh/id_ed25519.pub >> ~/.ssh/authorized_keys

# copy the new key to the clipboard
# if xclip is installed, copy to clipboard
if command -v xclip &> /dev/null
then
    cat ~/.ssh/id_ed25519.pub | xclip -selection clipboard

# print the ed25519 public key and tell user to add it to the git server
echo "Add the following public key to the github/gitlab/bitbucket repositories"
cat ~/.ssh/id_ed25519.pub

# clean up the temporary directory
rm -rf $tmpdir
