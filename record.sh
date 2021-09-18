#!/bin/bash

set -e

if [[ -z $1 ]]; then
  echo "plays the thingy and records it"
  echo "example:"
  echo -e "\t$0 packagename"
else
  rm -rf "recordings/$1/frames"
  mkdir -p "recordings/$1/videos"
  cargo run --release --package $1 -- -record
  filename="video$(( $(find recordings/$1/videos -type f -exec basename -s .mp4 {} \; | sed 's/^video//' | sort -n | tail -n1) + 1)).mp4"
  ffmpeg -framerate 60 -i "recordings/$1/frames/%03d.png" -pix_fmt yuv420p "recordings/$1/videos/$filename"
  echo "done"
fi
