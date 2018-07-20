#!/bin/bash

SLIDE_NUM="$1"
NUM_SLIDES="$(./last_slide.bash)"

if [ -z "$SLIDE_NUM" ]; then
    echo "Usage: ./add_slide.bash SLIDE_NUM" >&2
    exit 1
fi

if [ -z "$NUM_SLIDES" ]; then
    echo "Invalid number of slides" >&2
    exit 2
fi

for slide in $(seq "$NUM_SLIDES" -1 "$SLIDE_NUM"); do
    if [ -f "slides/$slide.html" ]; then
        mv "slides/$slide.html" "slides/$(expr "$slide" + 1).html"
    fi
done
