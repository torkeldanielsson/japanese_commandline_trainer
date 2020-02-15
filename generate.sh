#!/bin/bash
cargo fmt && cargo run -- $1 && pdflatex gen_kanji.tex && pdflatex gen_english.tex && pdflatex gen_samples.tex