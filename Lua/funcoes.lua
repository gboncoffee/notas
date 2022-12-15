-- Funções são cidadãos de primeira-classe. Inclusive, pode-se chamar tanto
-- funções escritas em Lua quanto em C. Toda a stdlib de Lua é justamente
-- escrita em C.
function factorial(x)
    if x == 0 then
        return 1
    else
        return x * factorial(x - 1)
    end
end
-- Ao chamar funções, os parênteses são opcionais somente se a função receber 1
-- argumento que seja uma string (literal, não conta variável) ou um construtor
-- de table.
--
-- Também pode declarar como anônimas:
factorial = function(x)
    -- código
end
--
-- Lua suporta funções com número de argumentos varíavel:
myprint = function(...)
    print(arg) -- a table arg carrega os argumentos de uma função assim além 
               -- de um parâmetro n mostrando quantos argumentos a função
               -- recebeu
end

-- E suporta closures, com funções tendo acesso às variáveis de suas funções
-- acima na stack. Dessa forma, podemos criar coisas como contadores:
new_counter = function()
    local i = 0
    return function()
        i = i + 1
        return i
    end
end
-- Sempre que chamar o que foi retornado por new_counter() (sem argumento nenhum
-- mesmo) ele irá retornar um valor incrementado em relação ao anterior.
