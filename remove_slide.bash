#!/bin/bash

SLIDE_NUM="$1"
NUM_SLIDES="$2"

if [ -z "$SLIDE_NUM" ] || [ -z "$NUM_SLIDES" ]; then
    echo "Usage: ./remove_slide.bash SLIDE_NUM NUM_SLIDES" >&2
    exit 1
fi

for slide in $(seq "$SLIDE_NUM" "$NUM_SLIDES"); do
    next_slide="slides/$(expr "$slide" + 1).html"

    if [ -f "$next_slide" ]; then
        mv "$next_slide" "slides/${slide}.html"
    else
        rm -f "slides/${slide}.html"
    fi
done
