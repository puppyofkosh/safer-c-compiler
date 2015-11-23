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
/usr/lib/gcc/i686-linux-gnu/4.8/collect2 --sysroot=/ --build-id --eh-frame-hdr -m elf_i386 --hash-style=gnu --as-needed -dynamic-linker /lib/ld-linux.so.2 -z relro /usr/lib/gcc/i686-linux-gnu/4.8/../../../i386-linux-gnu/crt1.o /usr/lib/gcc/i686-linux-gnu/4.8/../../../i386-linux-gnu/crti.o /usr/lib/gcc/i686-linux-gnu/4.8/crtbegin.o -L/usr/lib/gcc/i686-linux-gnu/4.8 -L/usr/lib/gcc/i686-linux-gnu/4.8/../../../i386-linux-gnu -L/usr/lib/gcc/i686-linux-gnu/4.8/../../../../lib -L/lib/i386-linux-gnu -L/lib/../lib -L/usr/lib/i386-linux-gnu -L/usr/lib/../lib -L/usr/lib/gcc/i686-linux-gnu/4.8/../../.. out/code.o -lgcc --as-needed -lgcc_s --no-as-needed -lc -lgcc --as-needed -lgcc_s --no-as-needed /usr/lib/gcc/i686-linux-gnu/4.8/crtend.o /usr/lib/gcc/i686-linux-gnu/4.8/../../../i386-linux-gnu/crtn.o
