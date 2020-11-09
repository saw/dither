#!/bin/bash
FILES=img/*
for f in $FILES
do
  echo "Processing $f file... out/${f}"
  ./target/release/dither $f -o "out/${f}"
  # take action on each file. $f store current file name
#   cat $f
done