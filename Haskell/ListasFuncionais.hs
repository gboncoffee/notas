-- Escritas assim:
list :: [1,2,3]

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
-- ++, concatena listas
[1,2,3] ++ [4,5,6] = [1,2,3,4,5,6]
