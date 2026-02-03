#!/usr/bin/env bash

# set -o errexit -o pipefail -o noclobber -o nounset

log="[$HOME/.scripts]"
debug_on="${DEBUG_SCRIPTS:-true}"

green_c='\033[1;32m'
red_c='\033[1;31m'
cyan_c='\033[1;36m'
yellow_c='\033[1;33m'
gray_c='\033[1;30m'
no_c='\033[0;37m'
current_c="${no_c}"
line="${gray_c}----------------------------------------------------------------------------${no_c}"
set_console_color() {
    current_c=${1:-}
    printf "$current_c" >&2
}
set_console_normal() {
    if [ "$current_c" != "${no_c}" ]
    then
        current_c=$no_c
        printf "${no_c}" >&2
    fi
}
trap set_console_normal EXIT

set_log() {
    log="$1"
}
debug() {
    if [[ ${debug_on} == "true" ]]; then
        (>&2 echo -e "${gray_c}$log $*${current_c}")
    fi
}
debug_cmd() {
    debug "${yellow_c}>>>${gray_c} $*"
}
info() {
    (>&2 echo -e "${cyan_c}$log $*${current_c}")
}
warn() {
    (>&2 echo -e "${yellow_c}$log $*${current_c}")
}
error() {
    (>&2 echo -e "${red_c}$log $*${current_c}")
}

prepend_stdout() {
    suffix=$1
    while IFS= read -r line; do echo -e "$suffix $line"; done
}
prepend_stderr() {
    suffix=$1
    while IFS= read -r line; do echo -e "$suffix $line" >&2; done
}

run_cmd_wrap() {
    debug_cmd "$@"
    # shellcheck disable=SC2068
    $@
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
        exit_error "Command failed with exit code $exit_code"
    fi
}

exit_success() {
    debug "${green_c}[action completed]${current_c}"
    exit 0
}

exit_error() {
    if [ -n "${1:-}" ]
    then
        error "Error: ${1:-}"
    fi
    shift
    set_console_color "$gray_c"

    for line in "$@"
    do
        info "$line"
    done
    error "[action aborted]"
    exit 1
}
