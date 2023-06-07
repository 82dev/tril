#!/bin/bash

clang -S -emit-llvm ../stdlib/stdlib.c -o ./stdlib.ll
clang stdlib.ll example.ll 