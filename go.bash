#!/bin/bash
FILES=img/*
for f in $FILES
do
  echo "Processing $f file... out/${f}"
  ./target/release/dither $f -o "out/${f}"
done