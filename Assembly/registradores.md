# Registradores

Registradores são os bolsos e a mesa de trabalho da CPU, como foi explicado em
`cpu.md`.

Aliás, periféricos também tem registradores, geralmente mais específicos
e confusos que os da CPU, porém geralmente a conversa com periféricos é feita
através do sistema operacional.

## Os registradores em si

### Especiais

- _instruction pointer_: o ponteiro de instrução aponta para a próxima instrução
  na memória e é atualizado a cada instrução realizada pela CPU.  
- _flags_: registradores de 1 bit que indicam coisas a CPU.  

### Registradores em 16 bits e 32 bits

#### De segmento

Registradores de segmento tanto em 16 quanto 32 bits são de 16 bits.

- CS: *code segment*. Segmento onde estão as instruções.  
- DS: *data segment*. Segmento onde está a memória em si, dados, do programa.
  Podem existir vários mas a CPU só trabalha com 1 por vez.  
- SS: *stack segment*. Segmento onde está a stack.  
- ES: *extra segment*. Segmento extra, serve pra guardar um endereço qualquer.  
- GS and FS: são clones de ES, só existem em 386 e x86.  

Quanto aos registradores gerais em 32 bits, há três classes: os registradores
gerais de 16 bits, os registradores gerais estendidos de 32 bits e os
registradores de 8 bits. Os registradores menores na verdade são regiões dentro
dos registradores de 32 bits.

8 registradores de 16 bits: AX, BX, CX, DX, BP, SI, DI e SP (que é menos geral
que os outros). Todos existem também no 8086, 8088 e 80286. Quando a Intel
passou para 32 bits, ela dobrou o tamanho de cada um desses registradores
e colocou um E na frente do nome (EAX, EBX, etc). 

A metade de baixo dos registradores de 32 bits EAX, EBX, ECX e EDX pode ser
chamada pelo nome de 16 bits. No caso ainda de AX, BX, CX e DX em 32 bits, cada
metade deles é dividida em pedaços de 8 bytes, que pode ser individualmente
referenciada mantendo a letra principal (A, B, C ou D) e adicionando um H para
a metade de cima e um L para a de baixo: AH, AL, BH, etc.

O registrador IP (ou EIP em 32 bits *protected mode*) tem a única e exclusiva
função de salvar a distância do segmento de código, ou seja, registrar junto ao
CS qual instrução estamos executando no momento. A cada instrução realizada, IP
é incrementado com o tamanho da mesma. O tamanho depende da arquitetura,
e é chamado de EIP em 32 bits.

IP é o único registrador que não pode ser lido ou setado diretamente. Existem
truques para descobrir seu valor mas isso não é tão útil.

O registador de 16 bits FLAGS (em e com 32 bits EFLAGS) salva valores de 1 bit
usados para validar testes (*control flow*) em programas. Quase nunca é usado
como uma coisa só, mas como bits individuais.

Vale notar que, em 64 bits, os registradores trocam o E pelo R.

#### EFlags

Aqui uma tabela com todas as flags de CPUs x86:

![Assembly/share/flags.png]

As flags mais importantes são as seguintes:

- **OF**: *Overflow flag*. Marca o overflow de uma operação com números com
  sinal.  
- **DF**: *Direction flag*. Usada pelo programa para informar à CPU se
  operações *string* são para cima na memória (0) ou para baixo (1).  
- **IF**: *Interrupt enable flag*. Setada ou pelo programa, ou pela CPU.
  Autoriza interrupções na execução de programas quando em contexto de
  multitarefa cooperativa (*real mode*). Pode ser alterada com as instruções STI
  e CLI. Em *protected mode*, com multitarefa preemptiva, tentar modificá-la em
  user mode fará o Linux te dar um SEGFAULT.  
- **TF**: *Trap flag*. Usada por debuggers para forçar a CPU a executar uma
  instrução somente e depois parar. Não muito útil para outros programas.  
- **SF**: *Sign flag*. Marca que a última operação força um operador a ficar
  negativo. Qualquer operação que positive seu operador vai zerar essa flag.  
- **ZF**: *Zero flag*: Marca que a última operação deu resultado zero. Muito
  útil com jump condicional.  
- **AF**: *Auxiliary carry flag*. Usada com aritmética BCD.  
- **PF**: *Parity flag**. Mostra se o número de bits 1 no byte menos
  significativo do resultado é par (1) ou ímpar (0). Usada para *parity
  checking* com portas seriais.  
- **CF**: *Carry flag*. Mostra se o resultado de uma operação com números sem
  sinal (shift ou adição) carrega um bit do operador.

Cada instrução afeta flags de maneira diferente. Quando for usá-las, leia
a referência da instrução para saber quais flags ela afeta. Na dúvida, assuma
que elas não as mexem.
