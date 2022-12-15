;
; build:
;
; nasm -f elf -g -F stabs eatsyscall.asm
; ld -o eatsyscall eatsyscall.o
;

SECTION .data   ; seção com dados inicializados

eat_msg: db "Hello, World!",10
eat_len: equ $-eat_msg

SECTION .bss    ; seção com dados não-inicializados

SECTION .text   ; código

global _start   ; especifíca onde começar (main)

_start:
    nop                 ; pro gdb
    mov eax, 4          ; sys_write syscall
    mov ebx, 1          ; file descriptor stdout
    mov ecx, eat_msg    ; passa o offset da mensagem
    mov edx, eat_len    ; passa o tamanho da mensagem
    int 80H             ; syscall pra escrever no stdout

    mov eax, 1          ; exit syscall
    mov ebx, 0          ; exit code
    int 80H             ; syscall de suicídio
