.section .data
.section .text
.globl _start
_start:
pushl %ebp
movl %esp, %ebp
popl %ebp
movl $0, %ebx
movl $1, %eax
int $0x80
