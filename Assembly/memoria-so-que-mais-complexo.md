# Memória, porém mais complexo

Traduzindo livremente o terceiro parágrafo da página 77 de _Assembly Step by
Step_, de Jeff Dunteman:

> _A ingênua objeção que existe para uma CPU executar instruções de máquina pode
> ser disposta de maneira simples: ela executa instruções de máquina assim que
> ela as têm em suas mãos eletrônicas. O_ ***real*** _trabalho de uma CPU,
> e o real desafio da linguagem Assembly, está em localizar as tais instruções
> de máquina e dados na memória. Qualquer idiota consegue aprender instruções de
> máquina (Muitos o fazem.)_ ***A habilidade da linguagem Assembly consiste em
> uma compreensão profunda de endereçamento de memória.*** _Qualquer coisa
> a parte disso são detalhes - e detalhes fáceis a esse ponto._

Resumindo, entender o endereçamento da memória é importante pra \*\*\*\*.

## Modelos

Há diferentes modelos de endereçar a memória em x86, porém há três principais.
Em x86 32 bits para GNU/Linux, nós vamos focar em um deles, pois os outros estão
caindo em desuso.

### Real Mode Flat Model (Modelo Plano de Modo Real)

O modelo mais antigo. A CPU pode enxergar 1mb, mais usando o truque do
*segmento:distância*. Se usar só um segmento de 64K, nem precisa se preocupar
com isso. Normalmente é o que ocorre: o sistema operacional seta todos os
registradores de segmento para o segmento que você vai usar e você não mexe
neles. Antigamente era possível programar coisas como o Word Star ou o Turbo
Pascal em menos de 64Kb (inclusive o Turbo Pascal era bem menor porque ele
compilava programas em memória então precisava de espaço para si e para
o programa de Pascal que seria compilado), porém hoje nós somos preguiçosos
e gastamos memória com *darkmode stylesheet* em site de receita e JavaScript pra
fazer pop-up.

### Real Mode Segmented Model (Modelo Segmentado de Modo Real)

O segundo modelo é provavelmente a coisa que mais dá raiva de estudar em todo
o mundo da ciência da computação.

Na época do Intel 8080, o CP/M-80 era o OS mais comum para o mesmo. O 8080 era
uma CPU de 8 bits, porém tinha uma saída de endereçamento de 16 bits, o que
endereçava 64kb. A época, as máquinas geralmente não passavam dos 4kb a 8kb.
O modelo de memória dele fazia o seguinte: o CP/M-80 era carregado no **topo**
da memória existente, e os programas eram, quando necessários, carregados
a partir do endereço 0100H. Os primeiros 256 bytes sobrando eram chamados de
prefixo do segmento de programa (_program segment prefix_) e possuia um punhado
de informações e um buffer para o I/O do programa.

Ao criar uma CPU de 16 bits, a 8086, a Intel queria continuar permitindo que
programas escritos para CP/M-80 do 8080 funcionassem na 8086. A 8086 podia
endereçar até incríveis 1mb (16x a capacidade da 8080), então a Intel fez com
que fosse possível um programa rodar inteiro dentro de um chunk de 64kb, que era
endereçado atráves de um registrador de segmento (_segment register_).

O problema disso foi que novos programas, feitos para o próprio 8086 e que não
precisariam do modelo segmentado precisaram então usar a memória em chunks de
64kb, e se precisassem trocar de segmentos, precisariam trocar os valores nos
_segment registers_.

Quando rodando em _real mode segmented model_, processadores x86 podem acessar
diretamente até 1mb de memória (100000H bytes), chamada de _real mode memory_.
Lembre-se que o último endereço de 100000H bytes é FFFFFH, pois o computador
começa a contar do zero.

CPUs de x86 32 bits conseguem endereçar 4gb de memória sem dividi-la em
segmentos menores. Quando operando em _protected mode flat model_, o segmento
**é** de 4gb. Para ter compatibilidade com software antigo de DOS, as CPUs
possuem o chamado _virtual-86 mode_, em que ela se limita a usar somente o que
a 8086 conseguia usar. Isso deu certo, de verdade.

Quando operando em _real mode segmented model_, CPUs de x86 32 bits se
configuram para endereçar apenas 1mb de memória. Elas usam somente 20 de seus 32
pinos de endereçamento. Porém, além disso, elas ainda se configuram para separar
tudo em chunks de 64kb. Literalmente, elas podem "enxergar" 1mb inteiro porém se
"vendam" para acessar apenas 64kb.

Nesse modo, seu programa consegue enxergar 1mb inteiro de memória, combinando um
endereço de segmento de 16 bits com uma distância também de 16 bits. Note que
o endereço de segmento 01H especifica os primeiro e segundo bytes, o endereço
02H especifica os terceiro e quarto, etc. Multiplicar um endereço de segmento
por 16 resultará no endereço real de 20 bits.

Lembre-se que em *real mode*, partes do sistema operacional (ou no caso de 8086
e 8088, *o sistema inteiro*), outros programas e estruturas de dados do sistema
estão carregados na memória junto com seu programa. Dá pra destruir partes
importantes do sistema, derrubando-o junto com seu programa se não usar os
segmentos direito. Eventualmente, a Intel criou o *protected mode* (modo
protegido) para o 80386, onde isso não é mais possível. No 80286 havia um modo
protegido rudimentar, porém nenhum sistema fazia grande uso dele.

### Protected Mode Flat Model (Modelo Plano de Modo Protegido)

O modelo mais moderno que está por trás de sistemas operacionais modernos como
o GNU/Linux e o Windows. Só funciona a partir das CPUs i386 que suportam
a arquitetura IA-32. É bem parecido com o _real mode flat model_.

CPUs da Intel tem implementado um bom modo protegido desde o surgimento da 386
em 1986. Até o Windows NT em 1994, nem o MS-DOS nem o próprio Microsoft Windows
conseguiam usá-lo direito. O Linux, porém, por não precisar manter
compatibilidade legada, tem feito excelente uso do modo protegido desde que
surgiu em 1992. Programas Assembly de modo protegido devem rodar tanto em Linux
qunato em Windows desde o NT.

O programa consegue enxergar todos os 4gb, e cada endereço tem 32 bits, assim
como todos os registradores de uso geral e o EIP. Os registradores de segmento
ainda existem, porém literalmente *fazem parte* do sistema operacional.
O sistema os usa para mapear onde os seus 4gb estão, seja na memória física ou
virtual (a memória física pode ser maior que 4gb). Memória virtual pode ser
criada "mapeando" áreas do disco para serem usadas como memória (o tal do swap
do Linux).

Para simplificar, é como se o seu programa recebesse 4gb de memória para usar,
porém o sistema protege partes dela, de modo que você não pode simplesmente
acessar qualquer coisa. Isso depende do sistema operacional (sendo um sistema de
modo protegido, claro). Em modo protegido, seu programa pode manipular um
endereço de qualquer lugar dentro dos 4gb, porém tentar ler de certos lugares
acarretará em um erro, pois aquela memória está protegida do seu programa.

Agora os registradores de segmento têm um novo papel: o sistema operacional os
usa para mapear a memória virtual do seu programa para a memória real. Você não
pode mexer neles: eles são protegidos.

O ponto chave modo protegido é que ele permite que mais de um programa rode ao
mesmo tempo. No antigo IBM PC, o periférico de vídeo tinha memória embutida, que
era acessível universalmente, de forma que era possível exibir o que quiser na
tela simplesmente modificando esse buffer. Se isso ocorresse em um sistema
multitarefa, seria um caos. Então a memória é protegida e o kernel faz a ponte
entre os periféricos e os programas, por bibliotecas e drivers rodando em kernel
space.
