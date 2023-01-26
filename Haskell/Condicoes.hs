-- Haskell tem um if, mas é feião:

doubleSmallNumber x = if x >= 100 -- lembra da indentação de acordo com a coluna
                         then x
                         else x * 2
-- O if é uma expressão e precisa retornar algo, por isso o else é obrigatório.
-- Também pode ser um one-liner:
doubleSmallNumber x = if x >= 100 then x else x * 2

-- Uma maneira mais funcional de condicionar é com funções que se aplicam a
-- diferentes argumentos:
factorial 0 = 1
factorial x = x * factorial (x - 1)

-- Ou ainda:
factorial x
        | x == 0 = 1
        | otherwise = x * factorial (x - 1)
