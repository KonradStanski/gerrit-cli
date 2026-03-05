#!/bin/sh
set -eu

REPO="KonradStanski/gerrit-cli"
INSTALL_DIR="${GERRIT_INSTALL_DIR:-/usr/local/bin}"

main() {
    need_cmd curl
    need_cmd tar
    need_cmd uname

    local os arch target tag url tmpdir

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)  os="unknown-linux-gnu" ;;
        Darwin) os="apple-darwin" ;;
        *)      err "Unsupported OS: $os" ;;
    esac

    case "$arch" in
        x86_64|amd64)   arch="x86_64" ;;
        arm64|aarch64)  arch="aarch64" ;;
        *)              err "Unsupported architecture: $arch" ;;
    esac

    target="${arch}-${os}"

    printf "Detected platform: %s\n" "$target"

    # Get latest release tag
    tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d '"' -f 4)"

    if [ -z "$tag" ]; then
        err "Could not determine latest release"
    fi

    printf "Latest release: %s\n" "$tag"

    url="https://github.com/${REPO}/releases/download/${tag}/gerrit-${tag}-${target}.tar.gz"

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    printf "Downloading %s...\n" "$url"
    curl -fsSL "$url" -o "${tmpdir}/gerrit.tar.gz"

    tar xzf "${tmpdir}/gerrit.tar.gz" -C "$tmpdir"

    # Find the binary inside the extracted directory
    local binary
    binary="$(find "$tmpdir" -name gerrit -type f | head -1)"

    if [ -z "$binary" ]; then
        err "Could not find gerrit binary in archive"
    fi

    chmod +x "$binary"

    if [ -w "$INSTALL_DIR" ]; then
        mv "$binary" "${INSTALL_DIR}/gerrit"
    else
        printf "Installing to %s (requires sudo)...\n" "$INSTALL_DIR"
        sudo mv "$binary" "${INSTALL_DIR}/gerrit"
    fi

    printf "\ngerrit %s installed to %s/gerrit\n" "$tag" "$INSTALL_DIR"
    printf "Run 'gerrit --help' to get started.\n"
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "Required command not found: $1"
    fi
}

err() {
    printf "error: %s\n" "$1" >&2
    exit 1
}

main
