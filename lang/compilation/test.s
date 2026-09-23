.globl main
main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $12, %rsp
        movl    $26, %eax
        cdq
        movl    $2, %r10d
        idivl   %r10d
        movl    %eax, -4(%rbp)
        movl    $7, -8(%rbp)
        subl    $6, -8(%rbp)
        movl    -4(%rbp), %r10d
        movl    %r10d, -12(%rbp)
        movl    -8(%rbp), %ecx
        sarl    %cl, -12(%rbp)
        movl    -12(%rbp), %eax
        movq    %rbp, %rsp
        popq    %rbp
        ret
