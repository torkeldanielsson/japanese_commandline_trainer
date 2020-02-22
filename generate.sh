#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -x

rm -f gen_*

cargo fmt
cargo run -- $1

for filename in gen_kanji_*.tex; do
    pdflatex $filename
done

rm -f gen_*.aux
rm -f gen_*.tex
rm -f gen_*.log
