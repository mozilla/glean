#!/bin/sh

# Based on install.sh from https://github.com/japaric/trust
# License: MIT/Apache.
# See:
# * https://github.com/japaric/trust/blob/master/LICENSE-MIT
# * https://github.com/japaric/trust/blob/master/LICENSE-APACHE

set -e

help() {
    cat <<'EOF'
Install a binary release of a Rust crate hosted on github.com

Usage:
    crate-ci.sh [options]

Options:
    -h, --help      Display this message
    --git SLUG      Get the crate from "https://github.com/$SLUG"
    -f, --force     Force overwriting an existing binary
    --crate NAME    Name of the crate to install (default <repository name>)
    --tag TAG       Tag (version) of the crate to install (default <latest release>)
    --target TARGET Install the release compiled for $TARGET (default <`rustc` host>)
    --to LOCATION   Where to install the binary (default ~/.cargo/bin)
EOF
}

say() {
    echo "install.sh: $1"
}

say_err() {
    say "$1" >&2
}

err() {
    if [ -n "$td" ]; then
        rm -rf "$td"
    fi

    say_err "ERROR $1"
    exit 1
}

need() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "need $1 (command not found)"
    fi
}

force=false
while test $# -gt 0; do
    case $1 in
        --crate)
            crate=$2
            shift
            ;;
        --force | -f)
            force=true
            ;;
        --git)
            git=$2
            shift
            ;;
        --help | -h)
            help
            exit 0
            ;;
        --tag)
            tag=$2
            shift
            ;;
        --target)
            target=$2
            shift
            ;;
        --to)
            dest=$2
            shift
            ;;
        *)
            ;;
    esac
    shift
done

# Dependencies
need basename
need curl
need install
need mkdir
need mktemp
need tar

# Optional dependencies
if [ -z "$crate" ] || [ -z "$tag" ] || [ -z "$target" ]; then
    need cut
fi

if [ -z "$tag" ]; then
    need rev
fi

if [ -z "$target" ]; then
    need grep
    need rustc
fi

if [ -z "$git" ]; then
    # Markdown-style backticks
    # shellcheck disable=SC2016
    err 'must specify a git repository using `--git`. Example: `install.sh --git japaric/cross`'
fi

url="https://github.com/$git"
say_err "Git repository: $url"

if [ -z "$crate" ]; then
    crate=$(echo "$git" | cut -d'/' -f2)
fi

say_err "Crate: $crate"

url="$url/releases"

if [ -z "$tag" ]; then
    latest_url=$url/latest
    tag_url=$(curl -Ls -o /dev/null -w "%{url_effective}" "$latest_url")
    tag=$(echo "$tag_url" | rev | cut -d'/' -f1 | rev)
    if [ -z "$tag" ]; then
        err "Failed to get tag from $latest_url"
    fi
    say_err "Tag: latest ($tag)"
else
    say_err "Tag: $tag"
fi

if [ -z "$target" ]; then
    target=$(rustc -Vv | grep host | cut -d' ' -f2)
fi

say_err "Target: $target"

if [ -z "$dest" ]; then
    dest="$HOME/.cargo/bin"
fi

say_err "Installing to: $dest"

url="$url/download/$tag/$crate-$tag-$target.tar.gz"

td=$(mktemp -d || mktemp -d -t tmp)
curl -sL "$url" | tar xz -f - -C "$td"

for f in "$td"/*; do
    test -x "$f" || continue
    test -f "$f" || continue

    if [ -e "$dest/$(basename "$f")" ] && [ "$force" = false ]; then
        err "$f already exists in $dest"
    else
        mkdir -p "$dest"
        install -m 755 "$f" "$dest"
    fi
done

rm -rf "$td"
