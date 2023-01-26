-- Tuplas são exatamente o que se espera: listas fixas em tamanho e com
-- possibilidade de abrigar diferentes tipos. Tuplas tem um tipo próprio, então
-- É impossível, por exemplo, ter uma lista com tuplas (Int, String) e
-- (Int, Int, Int).

-- Funções que operam em pairs (tuplas com dois elementos):
--
-- fst, retorna o primeiro:
fst (1,2) == 1
-- snd, retorna o segundo:
snd (1,2) == 2
-- zip produz uma lista de pairs a partir de duas listas:
zip [1,2] [3,4] == [(1,3),(2,4)]
-- Caso uma lista seja maior que a outra, será cortada.