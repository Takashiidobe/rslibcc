#!/bin/bash

set -euo pipefail

# build the shlib
cargo b

# Compile the C++ library
clang++ src/main.cc -o build/demo

# Run the demo
./build/demo

# Print tests passed
echo "OK"
