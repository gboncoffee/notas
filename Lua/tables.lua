-- Lua tables são a única estrutura de dados da linguagem, e são poderosas.
--
-- São arrays anônimos e associativos, tratados como objetos e não primitivos.
-- Pacotes (libs) inclusive são representados como tables. Se em C tudo é uma
-- sequência de bytes, em Java tudo é uma classe e em Ruby tudo é um objeto, em
-- Lua tudo é uma table. Uma mesa. Uma tabela.
--
-- Pense em tables como um objeto alocado dinamicamente, de maneira que seu
-- programa só manipule ponteiros para ele. É claro, são passadas por
-- referência.
--
-- Uma table só é dealocada se o programa não tiver mais referências para ela.
-- Isso é:
a = { x = "Hello" }
b = a       -- b aponta para a mesma tabela que a
print(b.x)  -- b.x é "Hello"
b.x = "Foo" -- a.x vira "Foo" porque a e b são a mesma table
a = nil     -- a table ainda existe porque b tem uma referência para ela
b = nil     -- agora a table será dealocada eventualmente

-- Cuidado para não confundir os três:
a["x"]
a.x
a[x]
-- o primeiro e o segundo são a mesma coisa, a.x é só açúcar para a["x"], porém
-- a[x] irá usar o valor da variável x para indexar na table a.

-- Para usar um array comum, simplesmente use uma table com elementos com
-- indíces númericos:
a = {}
for i = 1,10 do
    a[i] = io.read()
end

-- tables também podem ser construídas como arrays, cujo primeiro item tem
-- índice 1:
a = { "hello", "world", "foo", "bar" }
-- tables são iluministas: sua construção só afeta sua inicialização, tirando
-- isso, todas se comportam da mesma maneira.

-- Lua sempre cria uma nova table quando atinge um construtor, então pode-se
-- criar linked lists assim:
list = nil
for line in io.lines() do
    list = { next = list, value = line }
end
-- Note que essa linked list ficará com a ordem reversa ao loop e funciona
-- porque o list do loop anterior não sai de escopo no próximo.
