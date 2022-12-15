#/usr/bin/lua
-- Lua é ridiculamente simples
--
-- LuaJIT é mais performática que a Lua da PUC-Rio, sendo conhecida como a
-- linguagem dinâmica mais rápida do mundo.
--
-- Comentários com --
--
-- Aliás, perceba que caso a primeira linha do arquivo comece com # o
-- interpretador ignora, justmente para permitir o uso de shebang em Unix.
--[[
Blocos de comentário assim. Bem útil porque se adicionar um - ao início do
bloco, ele descomenta tudo:
--]]
---[[
print "Hello, World!"
--]]

--
-- Lua é case-sensitive.
--
-- Essas são as palavras reservadas:
--[[
and       break     do        else      elseif
end       false     for       function  if
in        local     nil       not       or
repeat    return    then      true      until
while
--]]

-- 
-- Lua tem oito tipos: nil, boolean, number, string, userdata, function, thread
-- e table. nil é o tipo nulo. Use type() para ver o tipo das coisas:
print(type("Hello world"))  --> string
print(type(10.4*3))         --> number
print(type(print))          --> function
print(type(true))           --> boolean
print(type(nil))            --> nil
print(type(type(nil)))      --> string, perceba que type() sempre retorna uma 
                            --  string independentemente do tipo que recebe
-- A tipagem é dinâmica, forte e "duck". Note que funções são cidadãos de
-- primeira-classe. Nil pode ser usado para deletar variáveis.
--
-- Quanto aos booleans, note que diferente de muitas linguagens, Lua considera 0
-- e "" como true. nil, porém, é false.
--

--
-- numbers são sempre de ponto flutuante, como em JavaScript. A galera da PUC
-- argumenta que atualmente é raríssimo ter problemas reais de matemática de
-- inteiros usando arredondamento de ponto flutuante. Não acho que estão errados
-- btw, até porque não tenho direito de achar nada - eles são do Tecgraf da
-- PUC-Rio - mas acho bom o NuBank tomar cuidado.
--
-- De operadores aritméticos, Lua tem +, -, * e / além de também usar - como
-- negação (e não !). Também há suporte relativo a ^.
--
-- E de operadores comparativos, <, >, <=, >=, ==, ~=. Perceba que ~= é o
-- negador, ao invés de !=.<=, >=, ==, ~=. Perceba que ~= é o negador, ao invés
-- de !=.
--
-- Note que Lua compara funções, tables e userdata por referência. Só strings e
-- numbers aceitam < e > (e <= e >=). strings são comparadas por ordem
-- alfanúmerica, e para evitar bugs, Lua não permite comparar com esses
-- operadores uma string com um number e vice-versa.
--
-- Os operadores lógicos são and, or e not. Lua usa short-cut evaluation, ou
-- seja, só roda a segunda expressão caso necessário para validar a operação.
--
-- Se and é false, então ele retorna o primeiro argumento e vice-versa. O
-- contrário ocorre para o or. and tem maior precedência que or.
--
-- Isso gera expressões úteis. Por exemplo:
if not x then x = v end  -- Se x for nil ou false, x vira um valor padrão v.
a and b or c             -- Equivalente ao operador ternário (a ? b : c).
max = (x > y) and x or y -- Escolhe o maior entre x e y.
--
-- A precedência é a seguinte:
--
-- ^
-- not -
-- * /
-- + -
-- ..
-- < > <= >= ~= ==
-- and
-- or
--

-- Lua suporta multiple assignments:
a, b = 10, 2
-- Dessa maneira, dá pra swappar "sem" uma váriavel intermediária:
a, b = b, a
-- Variáveis são globais por padrão. Para torná-las locais para o bloco em que
-- estão, usa-se:
local x = 1
-- perceba que tais variáveis fora de qualquer bloco não podem ser acessadas nem
-- pelo script principal, ou seja, declarar uma local no script principal
-- simplesmente não funciona.

-- Condições suportam elseif
if x == 1 then
    print(1)
elseif x == 2 then
    print(2)
else
    print("other")
end

-- for com range (suporta steps também):
a = {}
for i = 1,10 do -- a variável de controle é local sempre e 
    a[i] = i    -- expressões são evaluadas uma vez só
end
-- for com table:
for i,j in ipairs(a) do
    print(i .. ": " j)
end

-- while:
while true do
    print("hello!")   
end

-- repeat/until (do/while):
repeat
    line = io.read()
until line ~= ""

-- break e return podem ser usados para pular fora de um bloco
