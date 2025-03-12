# shellcheck disable=SC1090,SC2034,SC1091,SC2148,SC2296,SC2296
export fpath=("/home/$USER/.zsh/completions" "$fpath")

alias nzrc="nano ~/.zshrc"
alias szrc="source ~/.zshrc"
alias nsrc="nano ~/.ssh/config"

if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi

export ZSH="$HOME/.oh-my-zsh"

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
    tmux
    universalarchive
    sudo
    rsync
    zsh-syntax-highlighting
)

source "$ZSH/oh-my-zsh.sh"

[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh
ZSH_AUTOSUGGEST_USE_ASYNC="true"

setopt nocorrectall; setopt correct

source "$HOME/.zsh_aliases"

export PATH="$HOME/.scripts:$HOME/.local/bin:$PATH"
eval "$(thefuck --alias)"

GPG_TTY="$(tty)"
export GPG_TTY

# Ensure all zsh completions are loaded from ~/.zsh/completions
fpath+=("$HOME/.zsh/completions")

if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi
