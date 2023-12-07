#!/bin/bash

function run_test {
    cd "$1" || exit 1

    PROJECT_CLEANED="$(echo "$1" | awk -F'/' '{print $3}')"

    RESULT="$(cargo run -- sample.txt 2> /dev/null)"
    ANSWER="$(cat sample-answer.txt)"

    FAILED="0"

    if [ "$RESULT" = "$ANSWER" ];
    then
        echo "$PROJECT_CLEANED passed"
    else
        echo "$PROJECT_CLEANED failed, expected '$ANSWER' got '$RESULT'"
        FAILED="1"
    fi


    cd "../.." || exit 1
}


PROJECTS="$(find . -maxdepth 2 -mindepth 2 -type d -name "aoc-d*" | sort)"
EXIT_CODE="0"

for PROJECT in $PROJECTS
do
    run_test "$PROJECT"
    if [ "$FAILED" = "1" ];
    then
        EXIT_CODE=1
    fi
done

exit "$EXIT_CODE"
