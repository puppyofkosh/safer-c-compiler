#! /bin/bash

# Build the given cmm file to objects/asm.s
# then to executable a.out
as --32 out/code.s -o out/code.o

# You could use the below command if you want to define the _start function
# yourself
# dynamic-linker specifies our magical linker (we dont want default)
# lc means to link with the C libraries
# -m elf_i386 means emulate for x86
# THIS LINEvv
#ld -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 out/code.o -o a.out -lc


# Copied from the output of gcc -v. Links with standard C library
gcc out/code.o
