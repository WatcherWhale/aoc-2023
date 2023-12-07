#!/bin/bash

function run_test {
    cd "$1" || exit 1

    PROJECT_CLEANED="$(echo "$1" | awk -F'/' '{print $3}')"

    RESULT="$(cargo run -- sample.txt 2> /dev/null)"
    ANSWER="$(cat sample-answer.txt)"

    if [ "$RESULT" = "$ANSWER" ];
    then
        echo "$PROJECT_CLEANED passed"
    else
        echo "$PROJECT_CLEANED failed, expected '$ANSWER' got '$RESULT'"
    fi


    cd "../.." || exit 1
}


PROJECTS="$(find . -maxdepth 2 -mindepth 2 -type d -name "aoc-d*" | sort)"

for PROJECT in $PROJECTS
do
    run_test "$PROJECT"
done
