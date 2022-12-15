-- strings são o que geralmente são: sequências de caractéres. Em Lua, pode-se
-- colocar qualquer byte nelas. strings tem manejo de memória automático e são
-- imutáveis, para mudá-las precisa-se criar outra com as modificações:
a = "one string"
b = string.gsub(a, 'one', "another") -- pode-se delimitar strings com " ou '
print(a) -- one string
print(b) -- another string

-- Lua possuí as seguintes sequências de escape de C:
--[[
\a	bell
\b	back space
\f	form feed
\n	newline
\r	carriage return
\t	horizontal tab
\v	vertical tab
\\	backslash
\"	double quote
\'	single quote
\[	left square bracket
\]	right square bracket
--]]

-- Pode-se também criar strings de múltiplas linhas e sem interpretação de \:
[[
# Markdown title

`Hello, World!`

Foo **bar**.
]]

-- Lua converte strings para números e vice-versa sempre que se tenta realizar
-- uma operação não suportada por um lado. Por isso mesmo o operador de soma (+)
-- não pode concatenar strings. Veja que:
print("10" + "11")
-- mostra 21 e não 1011, agora:
print("10" .. "11")
-- mostra 1011.
--
-- Aliás, veja que se escrever .. logo após um numeral PRECISA separar com ' '
-- se não o interpretador ou o JITC vai achar que se trata ponto de um decimal:
print(10 .. "11")

-- Apesar disso, comparações entre tipos são sempre falsas. Lua não converte os
-- tipos em comparações:
if "10" == 10 then
    print "No! It's not!"
else
    print "Yes! It's not equal!"
end
-- Dessa maneira, precisa-se forçar a conversão com tostring() ou concatenando
-- com uma string vazia:
if "10" == (10 .. "") then
    print "Yes! It's equal!"
else
    print "No! It's equal!"
end
