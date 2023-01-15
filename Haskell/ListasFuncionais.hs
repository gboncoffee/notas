-- Escritas assim:
list :: [1,2,3]

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
-- ++, concatena listas
[1,2,3] ++ [4,5,6] = [1,2,3,4,5,6]

-- Retirar elementos de lista por índice:
"Hello, World" !! 1 == 'e'
-- Indexação começa do zero

-- Listas podem conter listas, que podem ter tamanho diferente porém devem ser
-- do mesmo tipo.

-- Comparação de listas com ==, <, >, <= e >= é feita em order. Cada elemento é
-- comparado com o correspondente. Se forem iguais, 
