--
-- Haskell é uma famosíssima e ridiculamente bonita linguagem funcional
--
-- Que escrever programas com elegância? Escreva em Haskell.
--

-- Tipagem forte. Tipos devem sempre começar com Maíscula e valores com
-- minúscula.

-- "Função simple tem tipo: mapeia Int para Int para Int para Int".
simple :: Int -> Int -> Int -> Int
simple a b c = a * (b + c)

-- Declaração de tipos de operadores devem colocá-los entre parênteses.
(+) :: Int -> Int -> Int

-- Funções tem sempre precedência sobre operadores. Algo assim, portanto:
f x + g x
-- É f(x) + g(x)
--
-- Operadores são só funções de qualquer forma.

-- Let mantém a visibilidade de algo dentro da função. Por exemplo:
x = let c = a - b + 2
    in f c + g y c

-- Mantém a equação c visível somente dentro da função x, de maneira que podemos
-- ter abstração dentro da função sem comprometer o global namespace. Não se
-- pode redefinir variáveis dentro de um mesmo namespace.

-- Tem operador ^ para exponenciar, ainda bem.
