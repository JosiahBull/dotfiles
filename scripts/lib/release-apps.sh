#!/usr/bin/env bash
# Shared release installation for configure.sh and update-all.

install_disk_saver() (
    set -o errexit -o pipefail
    # Use a subshell so the temporary-directory trap stays local to this install.
    local destination="${1:-$HOME/.local/bin/disk-saver}"
    local arch platform asset release_url tmp
    case "$(uname -m)" in
        arm64|aarch64) arch="aarch64" ;;
        x86_64) arch="x86_64" ;;
        *) echo "Unsupported disk-saver architecture: $(uname -m)" >&2; exit 1 ;;
    esac
    case "$(uname -s)" in
        Darwin) platform="macos-$arch" ;;
        Linux) platform="linux-$arch-musl" ;;
        *) echo "Unsupported disk-saver OS: $(uname -s)" >&2; exit 1 ;;
    esac

    asset="disk-saver-$platform"
    tmp="$(mktemp -d)"
    local staged=""
    trap 'rm -rf "$tmp"; if [ -n "$staged" ]; then rm -f "$staged"; fi' EXIT

    # Resolve latest once so the archive and checksum always use the same release.
    echo "Downloading disk-saver for $platform..."
    release_url="$(curl -fsSL --retry 3 -o /dev/null -w '%{url_effective}' \
        https://github.com/JosiahBull/disk-saver/releases/latest)"
    release_url="${release_url/\/tag\//\/download\/}"
    curl -fsSL --retry 3 -o "$tmp/$asset.tar.gz" "$release_url/$asset.tar.gz"
    curl -fsSL --retry 3 -o "$tmp/checksums-sha256.txt" "$release_url/checksums-sha256.txt"

    # Verify only our archive; the manifest also lists the other platforms.
    awk -v file="$asset.tar.gz" '$2 == file || $2 == "./" file { print }' \
        "$tmp/checksums-sha256.txt" > "$tmp/checksum.txt"
    [ -s "$tmp/checksum.txt" ] || { echo "No checksum found for $asset.tar.gz" >&2; exit 1; }
    (
        cd "$tmp" || exit
        if command -v sha256sum &> /dev/null; then
            sha256sum -c checksum.txt
        else
            shasum -a 256 -c checksum.txt
        fi
    )
    tar -xzf "$tmp/$asset.tar.gz" -C "$tmp" "$asset/disk-saver"
    # Replace atomically, including when a scheduled Linux process is running.
    mkdir -p "$(dirname "$destination")"
    staged="$(mktemp "$(dirname "$destination")/.disk-saver.XXXXXX")"
    install -m 755 "$tmp/$asset/disk-saver" "$staged"
    "$staged" --version
    mv -f "$staged" "$destination"
)

install_ff_router() (
    set -o errexit -o pipefail
    local installer="${FF_ROUTER_INSTALLER:-}"
    local tmp=""
    trap 'if [ -n "$tmp" ]; then rm -rf "$tmp"; fi' EXIT
    if [ -z "$installer" ]; then
        tmp="$(mktemp -d)"
        installer="$tmp/ff-router-installer"
        echo "Downloading ff-router installer..."
        curl -fsSL --retry 3 -o "$installer" \
            https://github.com/josiahbull/ff-router/releases/latest/download/ff-router-installer
        chmod +x "$installer"
    fi

    # Extra flags let updates avoid the default-browser prompt. The installer
    # reuses the existing config without rewriting it.
    "$installer" --non-interactive "$@"
)

update_personal_apps() {
    local disk_saver_bin
    disk_saver_bin="$(command -v disk-saver || true)"
    if [ -z "$disk_saver_bin" ] && [ -x "$HOME/.local/bin/disk-saver" ]; then
        disk_saver_bin="$HOME/.local/bin/disk-saver"
    fi
    if [ -n "$disk_saver_bin" ]; then
        install_disk_saver "$disk_saver_bin"
    fi

    if [ "$(uname -s)" = Darwin ] && {
        [ -d "$HOME/Applications/ff-router.noindex/Firefox Router.app" ] ||
        [ -d "$HOME/Applications/Firefox Router.app" ];
    }; then
        install_ff_router --no-set-default
    fi
}
