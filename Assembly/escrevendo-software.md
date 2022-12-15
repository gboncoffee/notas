# Por favor comente

Especialmente em Assembly onde no dia seguinte você não vai fazer ideia de como
seu código funciona.

## Objetos e linkers

Normalmente, assemblers escrevem arquivos chamados objetos, que contém todas as
instruções. Em sistemas antigos como o DOS, tais arquivos gerados por assemblers
são executáveis *per se*, porém em sistemas Linux e Windows modernos, mais
coisas são necessárias para se rodar binários.

O linker, além de juntar o código de vários objetos, também garante que as
funções chamadas entre objetos estejam corretas e que as referências à memória
*actually* referenciam algo. 

Tanto o Nasm (Netwide Assembler) quanto o ld (GNU Linker) possuem algumas
bandeiras. O último parâmetro da invocação de ambos deve ser sempre o/os
arquivos:

### Nasm

- `-f`: especifica o formato de saída, normalmente `elf`.  
- `-g`: para manter informações de debug no assembly.  
- `-F`: especifica o formato das informações de debug (stabs).  
- `-o`: padrão, especifica o arquivo de saída.  

### ld

- `-o`: padrão, especifica o arquivo de saída (lembrando que em Linux geralmente
  não se coloca extensão em binário).  
- `-m`: emulação, serve por exemplo com `elf_i386` para linkar 32 bits em um
  sistema de 64 bits.
