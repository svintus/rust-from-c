#!/bin/bash

rustc test.rs --crate-type=staticlib
clang test.c -o test -ltest -L$PWD

./test
