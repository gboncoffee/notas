-- Escritas assim:
list :: [Int]
list = [1,2,3]

-- Strings também são apenas listas de caractéres.

-- A lista vazia [] chama-se nil.

-- Adicionar elementos em frente a lista:

0 : list

-- Na verdade, [1,2,3] é a mesma coisa que:
1 : (2 : (3 : []))
-- Que é igual a: pois : é "right-associative"
1 : 2 : 3 : []

-- Funções em listas:
--
-- head, retorna o primeiro elemento da lista funcional:
head [1,2,3] == 1
-- tail, retorna o último/segundo elemento da lista funcional:
tail [1,2,3] == [2,3]
-- last, retorna o último último mesmo:
last [1,2,3] == 3
-- init, retorna tudo menos o último mesmo:
init [1,2,3] == [1,2]
-- length, retorna o tamanho da lista:
length [1,2,3] == 3
-- null, vê se a lista é nula:
null [] == True
-- reverse, reverte a lista:
reverse [1,2,3] == [3,2,1]
-- take, retorna n elementos da lista:
take 1 [1,2,3] == [1]
-- drop, retorna a lista sem n elementos:
drop 1 [1,2,3] == [2,3]
-- maximum e minimum, bem straighforward:
maximum [1,2,3] == 3
minimum [1,2,3] == 1
-- sum, soma listas:
sum [1,2,3] == 6
-- product, similar:
product [1,2,3] == 6
-- elem, verifica se uma lista possui certo elemento. Geralmente chamada como 
-- infixa por legibilidade:
4 `elem` [1,2,3] == False
-- ++, concatena listas
[1,2,3] ++ [4,5,6] = [1,2,3,4,5,6]

-- Retirar elementos de lista por índice:
"Hello, World" !! 1 == 'e'
-- Indexação começa do zero

-- Listas podem conter listas, que podem ter tamanho diferente porém devem ser
-- do mesmo tipo.

-- Comparação de listas com ==, <, >, <= e >= é feita em ordem. Cada elemento é
-- comparado com o correspondente. Se forem iguais, compara-se o próximo, e
-- etc.

-- Listas suportam ranges:
list = [1..20]
-- Inclusive, uma lista pode começar de certo ponto e nunca terminar, porque
-- Haskell é lazy:
list = [1..]
-- Suportam também ranges aritméticos:
[2,4..10] == [2,4,6,8,10]

-- Funções que criam listas infinitas:
--
-- cycle, que repete uma sequência infinitamente:
take 20 $ cycle [1,2,3]
-- repeat, que recebe um elemento e cria uma lista infinita com ele:
take 20 $ repeat 5
-- geralmente é melhor usar replicate para pegar n elementos repetidamente:
replicate 20 5

-- Haskell suporta list comprehension semelhante à set comprehension da
-- matemática. Exemplo:
twoExponential = [2^x | x <- [0..]] -- lista de todas as potências de 2
-- Tem como colocar predicados depois:
divisors342 = [ x | x <- [1..342], 342 `mod` x == 0 ] -- lista com todos os
                                                      -- divisores de 342
-- Tem como pegar valores de mais de uma lista. Nesse caso, o resultado será
-- uma permutação das listas (i.e., duas listas de 4 elementos resultará numa
-- lista de 16):
[ x*y | x <- [2,5,10], y <- [8,10,11]]

-- Não tente usar list comprehension com mais de uma lista infinita. Não tem
-- como permutar listas infinitas, e o programa vai ficar travado num loop
-- infinito.