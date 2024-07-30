#!/usr/bin/env bash

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
    if [ -d ~/.local/share/bin/ ]; then
        rm -rf ~/.local/share/bin/commitia
        rm -rf ~/.local/share/bin/locales
    else
        mkdir -p ~/.local/share/bin/
    fi
    cp -rf dist/commitia.js ~/.local/share/bin/commitia
    cp -rf locales/ ~/.local/share/bin/
else
    echo "The bun build command failed."
    exit 1
fi
