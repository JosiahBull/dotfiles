#!/bin/bash
# shellcheck disable=SC1091

set -o errexit -o pipefail -o noclobber

tmpdir=$(mktemp -d)

# clone the current repository into a temporary directory, recursively with submodules
git clone https://github.com/JosiahBull/dotfiles "$tmpdir"
pushd "$tmpdir"
git submodule update --init --recursive --depth 2
./configure.sh
popd
