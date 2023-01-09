section .data

section .text

global _start

_start:

    ;
    ; 
    ; Cópia e movimentação de dados
    ;
    ;
    mov eax, 1     ; registrador <- dado imediato
    mov ebx, edi   ; registrador <- registrador
    mov [ebp], edi ; memória     <- registrador
    mov edx, [esi] ; registrador <- memória
    ;
    ; Não é preciso lembrar que não tem como mover um dado maior que o
    ; registrador para dentro dele. Mas é possível mover qualquer dado até o
    ; tamanho dele. Algo assim por exemplo é possível:
    mov eax, "asdf"
    ; Porém, por causa da endianess do x86, o texto ficará "ao contrário".
    ;
    ; Copia, porém move o sign bit caso necessário. Serve para mover entre
    ; registradores de diferentes tamanhos. O destino deve sempre ser um
    ; registrador, a fonte pode ser memória.
    movsx eax, bx

    ;
    ;
    ; Aritmética binária geral
    ;
    ;
    ; Instruções de incremento e decremento.
    ;
    inc eax
    dec eax
    ; Ambas tem só um operando. A instrução INC não seta a flag CF, mesmo que dê
    ; overflow. Quando dá, volta a zero.
    ;
    ;
    ; Outras:
    ;
    ; Nega o valor (Gera o seu complemento, seu valor negativo ou seu valor
    ; positivo).
    neg eax
    ;
    ; Soma os valores e guarda o resultado no primeiro registrador.
    add eax, ebx
    ;
    ; Subtrai o segundo do primeiro e guarda o resultado no primeiro.
    sub eax, ebx

    ;
    ;
    ; Control flow
    ;
    ;
    ; Instruções de jump:
    ;
    ; Geralmente pulam para labels:
foo: mov eax, 1
    ;
    ; jump sempre, sem condição nenhuma:
    jmp foo
    ;
    ; jump se ZF não estiver setada:
    jnz foo
    ;

section .bss
