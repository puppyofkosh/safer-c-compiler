# safer-c-compiler

----------------------------------------------
### Dependencies

Currently, the compiler only supports Linux because it needs the library and x86 assembler from gnu gcc.

For 64-bit Linux, gcc-multilib should be installed, because the target assembly code is in 32-bit form.

Install in Ubuntu:
```bash
sudo apt-get install gcc-multilib
```

If you want to use valgrind on the executables this produces
and you're on 64 bit you will need to do:
```bash
sudo apt-get install libc6-dbg:i386
```

(See [the question to get more information](https://bugs.launchpad.net/ubuntu/+source/eglibc/+bug/881236) )

----------------------------------------------------------
### How to run
```bash
cargo run [program.sc] # this will give the assembly code.s in the folder 'out'
./build.sh
./a.out
```

Alternatively you can do:
```bash
./run.sh demo/hello_world.sc
```

----------------------------------------------------------
### How to run the tester
```bash
python tester.py
```
-----------------------------------------------------------
### Functionality
1. Supported operators
  * Arithmetic:(+,-,*,/)
  * Logic:(==, >, <, >=, <=, !=)
2. Reserved words
  * while, print, if, else, main, struct, int, char
3. Supported types
  * int
  * char
  * int*
  * char*
4. Supported type of constants
  * string constant like "foo"
  * char constant like 'd'
  * int constant like 12345
5. Significant difference from C
  * no for-loop
  * instead of malloc(), using allocate()
  * access array elements in *(a+i) manner
  * no && and || operators
  * struct definition in `Something a` manner instead of
  `struct Something a`
6. Ideas for more tests
  * call non existent function
  * function argument has meaningless type
  * dereferencing non pointer
  * getting unkown field
  * getting field of some meaningless thing (a function call)
  * wrong return type
  * passing structs to functions
  * returning struct from function
  * check type exists
  * pointer division/multiplication weird stuff
7. Tricky tests:
  * weird_reference_and_dereference.sc
  * pointer_of_a_pointer.sc

# Things we have done
A bunch of things here...
-else statements (don't worry about doing else if)
-functions with multiple args
-eliminate the stupid "call" keyword (instead of call(function, a), just function(a))
-be able to get the address of a particular struct field (ex: &x.y)
-change syntax to be C like
-be able to call malloc() and free(), etc along with other library routines
-individual characters (example: 'a')

# To do list (stuff we need to do!)
Stuff we need to do to have a "C" compiler:
-more tests (test all possible errors, and also write longer/more complicated programs)
-maybe allow forward declarations
-array reference (be able to do a[i]). Alternatively implement pointer arithmetic and just do *(a + i)
-operators && and ||
-error messages from the parser


Stuff we'd like to have:

-dereference arbitrary expression (eg *(a + f(b))).
Right now you just do p = a + f(b) and then dereference p
-function pointers
-null keyword
-simple optimizer which gets rid of redundant
instructions like a push immediately followed by a pop
-break keyword
-0 arg functions
-negative numbers! Right now we just do x = (0 - n)

Safety stuff:
-Check that a pointer isn't assigned something that'll go out of scope before it does
-Implement unique_pointers who get freed when they go out of scope, and get moved when assigned
-check for uninitialized variables

figure out how to link with crt's _start function
safety
