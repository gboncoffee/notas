/*
 * Traits são similares a interfaces de linguagens verdadeiramente orientadas a objeto, permitindo
 * que diferentes tipos tenham funcionalidades semelhantes.
 *
 * Baseam-se na ideia de que diferentes tipos podem ter o mesmo comportamento quando pode-se chamar
 * os mesmos metódos com eles.
 *
 * Traits são extremamente poderosos e complexos, recomendo fortemente a leitura do capítulo 10.2
 * do The Book.
 */

pub trait Number {
    fn pow2(&self) -> u128;
}

/*
 * Declara-se o trait Number público, para que a crate que utilize da crate que o declare também o
 * possa utilizar, e declara-se o método que se deseje implementar.
 *
 * Cada tipo que implemente a trait Number é forçado a implementar um método pow e sqrt. O
 * compilador se recusará a compilar caso contrário.
 *
 * Para implementar a trait em um tipo, usa-se impl:
 */
pub struct Positive {
    pub value: u128,
}
impl Number for Positive {
    fn pow2(&self) -> u128 {
        self.value * self.value
    }
}

/*
 * Caso a trait seja declarada com um método definido e não só declarado, ele se torna o método
 * padrão para tipos que implementem tal trait.
 *
 * Mesmo assim, para sobreescrever um método já definido, só é preciso redefini-lo, sem precisar
 * marcar com @Override ou qualquer outra bullshit assim.
 *
 * Perceba ainda que um método padrão de uma trait pode chamar qualquer outro método que essa trait
 * defina, ainda que não tenha um método padrão.
 */

/*
 * O grande poder das traits se dá ao definir parâmetros que devem implementá-la em funções. Ao
 * definir tal função, não importa o tipo do parâmetro, desde que implemente a trait Number:
 */
pub fn pow2(item: &impl Number) -> u128 {
    item.value * item.value
}

/*
 * Para definir funções que recebam dois parâmetros que devem ser do mesmo tipo, porém qualquer um
 * que implemente a trait, usa-se:
 */
pub fn foo<T: Number>(a: &T, b: &T) {
    println!("{} {}", a, b);
}

/*
 * Também é possível exigir mais de uma trait usando +:
 */
pub fn pow2(item: &(impl Number + Display)) {
    println!("{}: {}", item.value, item.value * item.value);
}

/*
 * Surge um problema ao definirmos vários traits e diferentes tipos em uma função. Veja essa
 * declaração, por exemplo:
 */
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

}
/*
 * Para "limpá-la", podemos usar a cláusula where:
 */
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}

/*
 * Também é possível retornar tipos que implementem traits:
 */
fn num(num: &impl Number) -> impl Number {
    num
}
/*
 * Uma função assim, porém, continua podendo retornar só um tipo.
 */
