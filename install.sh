#!/usr/bin/env bash

uninstall_commitia() {
    local install_dir1="$HOME/.local/bin"
    local install_dir2="/usr/bin"

    if [ -d "$install_dir1" ]; then
        rm -rf "$install_dir1/commitia"
        rm -rf "$install_dir1/locales"
    fi

    if [ -d "$install_dir2" ]; then
        rm -rf "$install_dir2/commitia"
        rm -rf "$install_dir2/locales"
    fi

    if [ -d /tmp/commitia ]; then
        rm -rf /tmp/commitia
    fi

    echo "commitia has been uninstalled."
    exit 0
}

if [ "$(uname -s)" = "Linux" ] && [ -f /etc/os-release ] && grep -q "ID=nixos" /etc/os-release; then
    DEFAULT_INSTALL_DIR="/usr/bin"
else
    DEFAULT_INSTALL_DIR="$HOME/.local/bin"
fi

INSTALL_DIR="$DEFAULT_INSTALL_DIR"
for arg in "$@"; do
    case $arg in
        --uninstall)
            uninstall_commitia
            ;;
        --dir=*)
            INSTALL_DIR="${arg#*=}"
            shift
            ;;
    esac
done

if ! command -v bun &> /dev/null
then
    echo "bun is not installed. Please install it first."
    exit 1
fi

if command -v bun &> /dev/null
then
    bun install
fi

if bun build commitia.js --outdir=dist/ --format=esm --external=i18xs --external=picocolors --sourcemap; then
    if [ "$INSTALL_DIR" = "/usr/bin" ] && [ "$EUID" -ne 0 ]; then
        echo "Permission denied: You need to run this script as root to install to /usr/bin."
        exit 1
    fi

    mkdir -p "$INSTALL_DIR"

    if [ -d "$INSTALL_DIR/commitia" ]; then
        rm -rf "$INSTALL_DIR/commitia"
    fi

    if [ -d "$INSTALL_DIR/locales" ]; then
        rm -rf "$INSTALL_DIR/locales"
    fi

    cp -rf dist/commitia.js "$INSTALL_DIR/commitia"
    cp -rf locales/ "$INSTALL_DIR/"
else
    echo "The bun build command failed."
    exit 1
fi