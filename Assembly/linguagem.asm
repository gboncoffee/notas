section .data
foo: db "Hello, World!"

section .text

global _start

_start:

    ;
    ; Por convenção, em Assembly, o primeiro dos operandos é sempre o destino, e
    ; o segundo é a fonte.
    ;
    ; Os dados que podem ser tratados são dados da memória, dados imediatos
    ; (valores literais) e dados de registradores. Perceba que é possível passar
    ; dados entre registradores, de registradores para a memória e vice-versa,
    ; mas não é possível passar dados entre lugares da memória diretamente.
    ;
    ; A maioria das instruções não permite operar em dois lugares da memória: ou
    ; é memória com registrador, ou registrador com memória, ou registrador com
    ; registrador, etc.
    ;
    ; Para fazer a dereferência, coloca-se [] no nome do registrador:
    mov eax, [ebp]
    ; O que estiver dentro dos [] é o "endereço efetivo" que será usado. É
    ; possível fazer aritmética de ponteiro, por exemplo:
    mov ebx, [ebp + 4]
    ; ou ainda adicionar dois endereços (somente dois):
    mov ecx, [eax + ebx]
    ; ou ainda adicionar endereços e um número:
    mov edx, [eax + ebx + 11]
    ; Perceba ainda que, se tem um nome (por exemplo, em data), e usa-se isso:
    mov eax, foo
    ; o que será movido é o ENDEREÇO, porque em asm nomes representam endereços.
    ; Para mover o valor, deve dereferenciar. Caso o valor seja maior que o
    ; registrador, porém, ele será cortado.
    ;
    ; Caso queira trabalhar com valores menores que o que se tem guardado,
    ; deve-se usar um registrador de tamanho apropriado.
    ;
    ; É possível ainda usar especificadores de tamanho:
    mov [eax], byte "a"
    mov [ebx], word "as"
    mov [ecx], dword "asdf"

    ;
    ; Labels são escritos com dois pontos:
DoMore: dec eax
    jnz DoMore

section .bss
