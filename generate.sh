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

for filename in gen_english_*.tex; do
    pdflatex $filename
done

for filename in gen_samples_*.tex; do
    pdflatex $filename
done

rm gen_*.aux
rm gen_*.tex
rm gen_*.log
