#!/bin/bash

PROGRAMM="./target/debug/cat-rs"
SOURCE_FILE="./src/main.rs"

diff <($PROGRAMM $SOURCE_FILE $SOURCE_FILE) <(cat $SOURCE_FILE $SOURCE_FILE)