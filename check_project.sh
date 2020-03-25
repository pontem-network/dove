#!/usr/bin/env bash
set -e

# RustFMT
# install: rustup component add rustfmt --toolchain stable
if [ "$GIT_EDITOR" == ":" ]; then
  FORMATTED_FILES=$(cargo +stable fmt --all --message-format short)
  if [ "$FORMATTED_FILES" != "" ]; then
    git add $FORMATTED_FILES
  fi
else
  cargo +stable fmt --all
fi

# Clippy
# install: rustup component add clippy --toolchain stable
# clippy available in stable channel only
cargo +stable clippy --all --tests --examples -- -Dwarnings

cargo test --all --tests
