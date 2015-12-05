# safer-c-compiler

# Dependencies
For 64-bit Linux, gcc-multilib should be installed.
You can install it by entering: sudo apt-get install gcc-multilib

# How to run
cargo run [program.sc] --- this will give the assembly code.s in the folder 'out'
./build
./a.out
The output of the program will then show in the terminal.

# How to run the tester
python tester.py

# Things we have done
A bunch of things here...
-else statements (don't worry about doing else if)
-functions with multiple args
-eliminate the stupid "call" keyword (instead of call(function, a), just function(a))
-be able to get the address of a particular struct field (ex: &x.y)

# To do list (stuff we need to do!)
Stuff we need to do to have a "C" compiler:
-more tests (test all possible errors, and also write longer/more complicated programs)
-be able to call malloc() and free(), write(), etc along with other library routines
-maybe allow forward declarations
-array reference (be able to do a[i]). Alternatively implement pointer arithmetic and just do *(a + i)
-operators && and ||
-individual characters (example: 'a')
-error messages from the parser
-change syntax to be C like

Stuff we'd like to have:

-dereference arbitrary expression (eg *(a + f(b))).
Right now you just do p = a + f(b) and then dereference p
-function pointers
-null keyword
-simple optimizer which gets rid of redundant
instructions like a push immediately followed by a pop
-break keyword
-0 arg functions

Safety stuff:
-Check that a pointer isn't assigned something that'll go out of scope before it does
-Implement unique_pointers who get freed when they go out of scope, and get moved when assigned
-check for uninitialized variables

figure out how to link with crt's _start function
safety

[Reserved words]
return, print

[Supported Operator]
*,-,+,/
==, >, <, >=, <=, !=

[Supported Types]
int, char, pointer


ideas for more tests:
variable already declared
function argument type wrong
