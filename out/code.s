.section .data
decimal_format_str: .asciz "%d\n"
.section .text
.globl _start
_start:
pushl %ebp
movl %esp, %ebp
movl $0, %eax
pushl %eax
pushl $decimal_format_str
call printf
addl $8, %esp
pushl $0
call fflush
addl $4, %esp
movl $123, %eax
cmp $0, %eax
je L0
movl $456, %eax
cmp $0, %eax
je L1
movl $1, %eax
pushl %eax
pushl $decimal_format_str
call printf
addl $8, %esp
pushl $0
call fflush
addl $4, %esp
L1:
L0:
movl $0, %eax
popl %ebp
movl %eax, %ebx
movl $1, %eax
int $0x80
