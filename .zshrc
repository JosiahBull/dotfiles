# shellcheck disable=SC1090,SC2034,SC1091,SC2148,SC2296,SC2296

# Dotfiles location
export DOTFILES_DIR="${DOTFILES_DIR:-$HOME/.dotfiles}"

export fpath=("$HOME/.zsh/completions" "${fpath[@]}")

alias nzrc='nano $DOTFILES_DIR/.zshrc'
alias szrc="source ~/.zshrc"
alias nsrc='nano $DOTFILES_DIR/ssh_config'

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
    rust
)

source "$ZSH/oh-my-zsh.sh"

# Source p10k and aliases from dotfiles directory
[[ -f "$DOTFILES_DIR/.p10k.zsh" ]] && source "$DOTFILES_DIR/.p10k.zsh"
ZSH_AUTOSUGGEST_USE_ASYNC="true"

setopt nocorrectall; setopt correct

source "$DOTFILES_DIR/.zsh_aliases"

# Scripts from dotfiles, then local bin
export PATH="$DOTFILES_DIR/scripts:$HOME/.local/bin:$PATH"

GPG_TTY="$(tty)"
export GPG_TTY

if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi

export NODE_OPTIONS="--max-old-space-size=8192"
export CLAUDE_CODE_MAX_OUTPUT_TOKENS=64000
