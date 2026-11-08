#!/bin/sh
# Télécharge le test fonctionnel 6502 de Klaus Dormann (GPL, non redistribué ici).
set -e
DIR="$(cd "$(dirname "$0")/.." && pwd)/target/dormann"
mkdir -p "$DIR"
URL=https://github.com/Klaus2m5/6502_65C02_functional_tests/raw/master/bin_files/6502_functional_test.bin
curl -fsSL --max-time 60 -o "$DIR/6502_functional_test.bin" "$URL"
echo "OK : $DIR/6502_functional_test.bin"
echo "Lancez : cargo test --release -p apple1-core --test dormann -- --nocapture"
