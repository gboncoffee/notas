;
; Aqui tem o minímo necessário para o nasm assemblar e o ld linkar:
; 
; Para buildar algo assim, usa-se:
;
; nasm -f elf -g -F stabs eatsyscall.asm
; ld -o eatsyscall eatsyscall.o
;
; Caso esteja em Linux de 64 bits, seria necessário usar a bandeira -m elf_i386
; no ld para linkar como 32 bits
;
; Note que seria necessária a syscall de exit para um programa vazio terminar
; sem SEGFAULT. Sem essa syscall, o programa tenta executar coisas depois da
; seção de texto e o Linux não permite.
;
; Por causa dessa maneira de lidar com programas mal feitos inclusive que o
; Linux é bem seguro. Atualmente é extremamente improvável que se consiga
; estragar a própria máquina estudando Assembly (muito menos C).
;

section .data ; seção com dados inicializados

section .text ; seção de código em si

global _start ; declara nosso ponto de entrada: _start

_start:
    ; aqui vão as instruções do ponto de entrada do programa

section .bss  ; seção com dados não-inicializados, "block starting symbol"
              ; Váriaveis globais sem valor inicial
