#! /bin/bash

# Build the given cmm file to objects/asm.s
# then to executable a.out
as --32 out/code.s -o out/code.o

# dynamic-linker specifies our magical linker (we dont want default)
# lc means to link with the C libraries
# -m elf_i386 means emulate for x86
ld -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 out/code.o -o a.out -lc
