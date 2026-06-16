#! /bin/bash

rustc $1
./$2

rm $2
