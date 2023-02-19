-- 
-- Tipagem forte, claro.
--

--
-- Haskell faz números um pouco melhor que outras linguagens.
--
-- Possui o tipo Integer para números inteiros e uma lib para números racionais:
-- Ratio, com o tipo Rational. Flutuantes de precisão de uma palavra (Float) e
-- de duas palavras (Double). É importante lembrar da questão da matemática de
-- ponto flutuante em computadores binários, Haskell não resolve esse problema.
--
-- O tipo Integer é um big int, é representado com mais que uma word ou long
-- word se necessário. Pode crescer indefinidamente. Por isso, possui overhead.
-- O tipo inteiro "comum" é o Int, cujo tamanho varia de acordo com o
-- processador.
--

-- É possível criar type aliases:
type Octave = Int
-- Inclusive de pairs:
type Pitch = (PitchClass,Octave)

-- Algebraic data types são como Enums de Rust:
data PitchClass = Cf | C | Cs | Df | D | Ds | Ef | E | Es | Ff | F | Fs | Gf | G | Gs | Af | A | As | Bf | B | Bs
    deriving (Eq,Ord,Ix,Show,Read)

-- Também suportam polimorfismo como generics:
data Primitive a = Note Dur a
                 | Rest Dur
    deriving (Show,Eq,Ord)

-- É possível criar tipos bem complexos:
data Music a = Primitive (Primitive a)
             | Music a :+: Music a
             | Music a :=: Music a
    deriving (Show,Eq,Ord)
-- Nesse caso, Music pode ter 4 tipos:
-- - Um primitivo como definido antes;
-- - Um Music a ser tocado em sequência a outro;
-- - Um Music a ser tocado em paralelo a outro;

-- Qualquer coisa antes de => é um "Class constraint", como se fosse um
-- requisito de trait em Rust. Define qual/quais typeclasses um tipo genérico
-- deve implementar:
-- :t (==)
-- Eq(a) => a -> a -> Bool
--
-- Typeclasses importantes são:
-- - Eq para comparações;
-- - Ord para ordenação;
-- - Show para converter para String (como Display em Rust);
-- - Read para converter de String; 
-- - Enum para tipos ordenados numericamente. Podem ser usados em ranges;
-- - Bounded para tipos com valores máximo e mínimo;
-- - Num para tipos que podem se comportar como números;
