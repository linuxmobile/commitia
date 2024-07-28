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
    cp -rf dist/commitia.js ~/.local/share/bin/commitia
    cp -rf locales/ ~/.local/share/bin/
else
    echo "The bun build command failed."
    exit 1
fi
