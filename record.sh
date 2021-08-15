#!/bin/bash

set -e

if [[ -z $1 ]]; then
  echo "plays the thingy and records it"
  echo "example:"
  echo -e "\t$0 packagename"
else
  rm -rf "recordings/$1"
  cargo run --release --package $1 -- -record
  ffmpeg -framerate 60 -i "recordings/$1/%03d.png" -pix_fmt yuv420p "recordings/$1.mp4"
  echo "done"
fi
