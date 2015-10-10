.section .data
.section .text
.globl _start
_start:
pushl %ebp
movl %esp, %ebp
movl $8, %eax
pushl %eax
movl $3, %eax
pushl %eax
movl $1, %eax
pushl %eax
movl $2, %eax
popl %ebx
addl %ebx, %eax
popl %ebx
imull %ebx, %eax
popl %ebx
imull %ebx, %eax
popl %ebp
movl %eax, %ebx
movl $1, %eax
int $0x80
