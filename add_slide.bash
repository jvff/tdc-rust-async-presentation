#!/bin/bash

SLIDE_NUM="$1"
NUM_SLIDES="$2"

if [ -z "$SLIDE_NUM" ] || [ -z "$NUM_SLIDES" ]; then
    echo "Usage: ./add_slide.bash SLIDE_NUM NUM_SLIDES" >&2
    exit 1
fi

for slide in $(seq "$NUM_SLIDES" -1 "$SLIDE_NUM"); do
    if [ -f "slides/$slide.html" ]; then
        mv "slides/$slide.html" "slides/$(expr "$slide" + 1).html"
    fi
done
