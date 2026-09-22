.globl main
main:
        pushq   %rbp
        movq    %rsp, %rbp
        subq    $16, %rsp
        movl    $44, -4(%rbp)
        movl    -4(%rbp), %r11d
        imull   $3, %r11d
        movl    %r11d, -4(%rbp)
        movl    -4(%rbp), %eax
        cdq
        movl    $2, %r10d
        idivl   %r10d
        movl    %eax, -8(%rbp)
        movl    $5, -12(%rbp)
        subl    $1, -12(%rbp)
        movl    -8(%rbp), %r10d
        movl    %r10d, -16(%rbp)
        movl    -12(%rbp), %r10d
        addl    %r10d, -16(%rbp)
        movl    -16(%rbp), %eax
        movq    %rbp, %rsp
        popq    %rbp
        ret
