#!/usr/bin/env bash

PROGRAM_NAME="ctrmml-rng-instrument"

if [ -d target ]
then
  rm -rf target
fi

cargo build --release

if [ ! -f target/release/$PROGRAM_NAME ]
then
  echo "build failed?"
fi

if [ ! -d ~/.local/bin ]
then
  mkdir -p ~/.local.bin
fi

cp target/release/$PROGRAM_NAME ~/.local/bin/$PROGRAM_NAME

echo ""
echo "------------------------------------------"
echo "  $PROGRAM_NAME copied to ~/.local/bin/$PROGRAM_NAME"
echo "------------------------------------------"
echo ""
