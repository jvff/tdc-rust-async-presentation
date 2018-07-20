#!/bin/bash

last_slide="0"

for slide in slides/*.html; do
    slide_num="$(basename $slide .html)"

    if [ "$slide_num" -gt "$last_slide" ]; then
        last_slide="$slide_num"
    fi
done

echo "$last_slide"
