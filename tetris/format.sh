#!/bin/bash

FD_CMD=""

if command -v fd > /dev/null; then
    FD_CMD="fd"
elif command -v fdfind > /dev/null; then
    FD_CMD="fdfind"
else
    echo "fd not found in path :("
fi

$FD_CMD -e gd -x sed -i 's/^[[:space:]]*$//g'

gdformat scripts
