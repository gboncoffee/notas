-- Rust roubou a ideia da programação funcional. Pattern matching está em tudo:
--
-- Define-se uma lista com pattern matching:
listSum :: [Float] -> Float
listSum [] = 0
listSum (x : xs) = x + listSum xs
