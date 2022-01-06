alias nzrc="nano ~/.zshrc"
alias szrc="source ~/.zshrc"
alias nsrc="nano ~/.ssh/config"

if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

export GOROOT=/usr/local/go
export GOPATH=$HOME/go
export PATH=$GOPATH/bin:$GOROOT/bin:$PATH

export ZSH="/home/josiah/.oh-my-zsh"

ZSH_THEME="powerlevel10k/powerlevel10k"

PROMPT_EOL_MARK=""
HYPHEN_INSENSITIVE="true"
DISABLE_AUTO_UPDATE="true"
ENABLE_CORRECTION="true"
COMPLETION_WAITING_DOTS="true"

ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE='fg=#999'
ZSH_AUTOSUGGEST_STRATEGY=(completion)

plugins=(
    git
    grc
    cp
    urltools
    safe-paste
    tmux
    universalarchive
    sudo
    rsync
    zsh-syntax-highlighting
    zsh-autosuggestions
)

source $ZSH/oh-my-zsh.sh

[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh
ZSH_AUTOSUGGEST_USE_ASYNC="true"

export PATH=$PATH:~/MEGA/Projects/scripts

eval $(thefuck --alias)

#Set JAVA_HOME and PATH
export JAVA_HOME=$(dirname $(dirname $(readlink $(readlink $(which javac)))))
export PATH=$PATH:$JAVA_HOME/bin
export CLASSPATH=.:$JAVA_HOME/jre/lib:$JAVA_HOME/lib:$JAVA_HOME/lib/tools.jar

export PATH=~/llvm13.0/bin:$PATH

export PATH=$PATH:/usr/local/go/bin
#TEMP
export DN3010_GITHUB_SSH_KEY="$HOME/.ssh/id_rsa"
#TEMP
export RUST_LOG=debug

setopt nocorrectall; setopt correct

source $HOME/.zsh_aliases
