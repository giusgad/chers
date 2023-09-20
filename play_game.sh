#!/bin/bash

# This script takes in an argument which is a file where every line is a command that needs to be sent to the engine
# and makes the engine play a game sending the commands to the engine through a named pipe

if [ -z "$1" ]; then # check if the file argument is provided
    echo "need a file argument"
    exit
fi

# create the pipes and start the engine
mkfifo chers-input &> /dev/null
mkfifo chers-output &> /dev/null
./target/release/chers < chers-input | tee -p chers-output &
pid=$!

while read line; do
    echo $line
    echo $line > chers-input
    if [[ $line == go* ]]; then # if the command is go wait for the engine to respond with bestmove
        while read -r l; do
            if [[ $l == *bestmove* ]];then
                break;
            fi
        done < chers-output
    fi
    # sleep is needed because the engine takes some time to setup the position and perform other operations
    sleep 0.5
done < $1

kill $((pid-1)) # the -1 is due to the fact the the registered pid is tee, while we want to kill chers

# cleanup
rm chers-input chers-output
