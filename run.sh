#!/bin/bash

#Help
Help()
{
   # Display Help
   echo "Run the cemantix cheat to find the daily word"
   echo
   echo "Syntax: ./run.sh [-f|h]"
   echo "options:"
   echo "f     First use (no build and no libs installed)"
   echo "h     Print this Help"
   echo
}

Install()
{
    pip install -r api/requirements.txt
    cargo build --release
}

Run()
{
    echo run
    gnome-terminal -- chromedriver
    cd api/
    gnome-terminal -- uvicorn main:app --reload
    sleep 2
    cd ..
    ./target/release/cemantix_cheat
}

# Handle options
while getopts ":hf" option; do
    case $option in
        h)
            Help
            exit;;
        f)
            Install
            Run
            exit;;
        \?)
            echo "Error: Invalid option"
            exit;;
    esac
done

# Basic case
Run