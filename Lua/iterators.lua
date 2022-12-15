-- Iterators em Lua geralmente são escritos com closures
--
-- Exemplo com arrays que, diferente de ipairs(), retorna só o elemento, e
-- quando acabar a lista retorna nil:
list_iter = function(t)
    local i = 0
    local n = table.getn(t)
    return function()
        i = i + 1
        if i <= n then return t[i] end
    end
end

-- Como o for chama seu iterator enquanto esse não retornar nil, podemos usá-lo
-- normalmente:
for i in list_iter({1, 2, 3}) do
    print(i)
end
