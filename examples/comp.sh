#!/bin/bash

clang -S -emit-llvm ../stdlib/stdlib.c -o ./stdlib.ll
clang example.ll stdlib.ll