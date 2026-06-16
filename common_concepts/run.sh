#! /bin/bash

rustc src/main.rs &> /dev/null
./main

rm main
