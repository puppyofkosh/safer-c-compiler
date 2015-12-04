# safer-c-compiler

[Dependencies]
For 64-bit Linux, gcc-multilib should be installed.
You can install it by: sudo apt-get install gcc-multilib

[How to run]
cargo run [program.sc]
./build
./a.out
The output of the program will then show in the terminal.

[How to run the tester]
python tester.py

[To do list (stuff we need to do!)]
-Else statements
-pointer arithmetic
-functions with multiple args
-Eliminate "call()" and replace it with just being able to call functions
-structs
- short circuit functions/operators
- parser should maintain context and know what structs are valid
- should be able to/have to forward declare structs and functions
- add "null" which should be of type pointer()

figure out how to link with crt's _start function
safety