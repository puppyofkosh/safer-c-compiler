#! /bin/bash

echo "Program:"
cat $1

# run the compiler
echo "Running the compiler"
cargo run $1

# turn asm to machine code
./build.sh

echo "Running..."
echo "Output:"
# run the program
./a.out
