.section .data
decimal_format_str: .asciz "%d\n"
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
pushl %eax
pushl $decimal_format_str
call printf
addl $8, %esp
movl $0, %eax
popl %ebp
movl %eax, %ebx
movl $1, %eax
int $0x80
