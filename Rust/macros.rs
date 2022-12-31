/*
 * Rust tem dois tipos de macros: declarativos e procedurais (subdivididos em três tipos).
 */

/*
 * Aliás, normalmente Rust funciona como um compilador de múltiplas passagens, tanto que não é
 * necessário declarar funções antes de chamá-las (diferente de C). Porém macros devem ser
 * declarados antes de serem chamados.
 */

/*
 * Macros declarativos:
 *
 * São os mais comuns, substituem o código, como em C. Exemplo da construção de uma versão mais
 * simples de vec!, sem o código de otimização que pré-aloca a quantidade necessária de memória:
 */
#[macro_export] // torna o macro visível sempre que a crate entrar em escôpo.
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
/*
 * Funciona por pattern matching também. Tem-se uma expressão (no caso "( $( $x:expr ),* )") com um
 * código associado.
 *
 * Nessa expressão, primeiro se isola ela própria com (), depois declara-se uma váriavel com $.
 * Dentro de $(), $x:expr se torna uma váriavel que dá match em qualquer expressão de Rust. A
 * vírgula após serve para indicar que uma vírgula PODE aparecer após. * indica que o pattern dá
 * match em 0 ou mais coisas.
 */

/*
 * Procedurais:
 * - #[derive], especificam código adicionado com o atributo derive;
 * - Macros attribute-like, definem atributos customizados;
 * - Macros function-like, como em C.
 *
 * Complicados demais para explicar aqui. Leia o capítulo 19 do The Book.
 */
