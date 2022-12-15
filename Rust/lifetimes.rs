/*
 * O propósito de lifetimes é evitar referenciar dados que não se tem intenção de. Como quando se
 * faz aritmética de ponteiro errada em C e acaba acessando lugares da memória que não deveria.
 *
 * Basicamente, o compilador verifica o escopo das variáveis para verificar se alguma aponta para
 * valores que já saíram de escopo, etc.
 */

/*
 * Veja o código abaixo:
 */
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("asdf");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest string: {}", result);
}
/*
 * Isso não compila porque o compilador não consegue saber qual lifetime acabará depois da função
 * longest: se será o de x ou y.
 *
 * Para resolver, precisaremos adicionar um lifetime explícito as referências. Veja a sintaxe, com
 * ' e um nome em letras minúsculas:
 */
// &i32           referência normal
// &'a i32        referência normal com lifetime explícito
// &'a mut i32    referência mutável com lifetime explícito
/*
 * Adicionamos um parâmetro genérico de lifetime à função longets para expressar que o valor de 
 * retorno será válido enquanto ambos os valores recebidos sejam:
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*
 * Dessa forma, dizemos que o lifetime do retorno será igual ao menor lifetime recebido.
 *
 * Perceba que aqui não estamos alterando o lifetime de ninguém, somente estamos dizendo ao
 * compilador para rejeitar qualquer conjunto de parâmetros que não respeitem essas condições. Isso
 * é, se x ou y sair de escopo, o retorno também sai, independentemente de se ele é o valor que
 * saiu de escopo ou não.
 *
 * Veja também que isso só ocorre porque não há como saber de antemão qual valor a função retorna:
 * x ou y. Caso a função sempre retornasse um deles, não seria necessário especificar o lifetime.
 */

/*
 * Sem lifetimes explícitos, o compilador segue três regras para inferir o lifetime das
 * referências, e se ainda houverem referências sem lifetime, ele dá erro:
 * 
 * 1 - Cada parâmetro da função que é uma referência recebe um lifetime diferente;
 * 2 - Se só houver um, o output recebe seu lifetime;
 * 3 - Em métodos, se houver mais de um, porém ele referencia self, todo o output recebe o lifetime
 *   de self.
 */

/*
 * Lifetimes em estruturas devem ser declarados tanto no nome quanto no block impl:
 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
    }
}

/*
 * Uma declaração de lifetime especial é a 'static, que declara que a referência será válida por
 * todo o programa. Note que isso ocorre com literais, por exemplo, porque estão diretamente no
 * binário.
 */
let s: &'static str = "Binários são legais";
