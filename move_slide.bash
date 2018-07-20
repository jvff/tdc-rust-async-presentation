#!/bin/bash

OLD_SLIDE_NUM="$1"
NEW_SLIDE_NUM="$2"

if [ -z "$OLD_SLIDE_NUM" ] || [ -z "$OLD_SLIDE_NUM" ]; then
    echo "Usage: ./move_slide.bash OLD_SLIDE_NUM NEW_SLIDE_NUM" >&2
    exit 1
fi

if [ "$OLD_SLIDE_NUM" -lt "$NEW_SLIDE_NUM" ]; then
    mv "slides/$OLD_SLIDE_NUM.html" "moving_slide.html"

    for slide in $(seq "$OLD_SLIDE_NUM" "$(expr "$NEW_SLIDE_NUM" - 1)"); do
        next_slide="slides/$(expr "$slide" + 1).html"

        if [ -f "$next_slide" ]; then
            mv "$next_slide" "slides/${slide}.html"
        else
            rm -f "slides/${slide}.html"
        fi
    done

    mv "moving_slide.html" "slides/$NEW_SLIDE_NUM.html"
elif [ "$OLD_SLIDE_NUM" -gt "$NEW_SLIDE_NUM" ]; then
    mv "slides/$OLD_SLIDE_NUM.html" "moving_slide.html"

    for slide in $(seq "$(expr "$OLD_SLIDE_NUM" - 1)" -1 "$NEW_SLIDE_NUM"); do
        if [ -f "slides/${slide}.html" ]; then
            mv "slides/${slide}.html" "slides/$(expr $slide + 1).html"
        fi
    done

    mv "moving_slide.html" "slides/$NEW_SLIDE_NUM.html"
fi
