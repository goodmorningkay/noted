#!/bin/sh
set -eu

# Replace with your GitHub username/org and repository name.
REPO="goodmorningkay/noted"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

main() {
    os=$(uname -s)
    arch=$(uname -m)

    case "$os" in
        Linux) os=linux ;;
        Darwin) os=macos ;;
        MINGW*|MSYS*|CYGWIN*) os=windows ;;
        *) echo "unsupported OS: $os" >&2; exit 1 ;;
    esac

    case "$arch" in
        x86_64|amd64) arch=amd64 ;;
        arm64|aarch64) arch=arm64 ;;
        *) echo "unsupported arch: $arch" >&2; exit 1 ;;
    esac

    if [ "$os" = "windows" ]; then
        binary="noted.exe"
    else
        binary="noted"
    fi

    asset="noted-${os}-${arch}.tar.gz"
    url="https://github.com/${REPO}/releases/latest/download/${asset}"

    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT

    echo "Downloading ${asset}..."
    curl -fsSL "$url" -o "$tmpdir/$asset"

    mkdir -p "$INSTALL_DIR"
    tar -xzf "$tmpdir/$asset" -C "$tmpdir"
    install -m 755 "$tmpdir/$binary" "$INSTALL_DIR/$binary"

    echo "Installed to ${INSTALL_DIR}/${binary}"
    echo "Make sure ${INSTALL_DIR} is in your PATH."
}

main
