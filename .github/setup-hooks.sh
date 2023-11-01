#!/usr/bin/env sh

root="$(pwd)"
rm -rf "$root/.git/hooks"
ln -s "$root/.github/hooks" "$root/.git/hooks"
