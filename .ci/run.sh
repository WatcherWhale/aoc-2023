#!/bin/bash

function run {
    cd "$1" || exit 1

    PROJECT_CLEANED="$(echo "$1" | awk -F'/' '{print $3}')"
    RESULT="$(cargo run -- input.txt 2> /dev/null)"

    echo "$PROJECT_CLEANED: $RESULT"

    cd "../.." || exit 1
}


PROJECTS="$(find . -maxdepth 2 -mindepth 2 -type d -name "aoc-d*" | sort)"
EXIT_CODE="0"

for PROJECT in $PROJECTS
do
    run "$PROJECT"
done

exit "$EXIT_CODE"
