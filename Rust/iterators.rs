/*
 * Quanto a iterators, são igualmente métodos de primeira-classe. São lazy, ou seja, só são
 * evaluados quando necessário.
 */

// A trait Iterator é definida assim:

pub trait Iterator {
    type Item;

    fn next(&self) -> Option<Self::Item>;
}

/*
 * Quando um for roda um iterator, chama o método Iterator.next até que retorne None. Caso
 * queiramos iterar "manualmente", podemos inclusive chamar o método next diretamente:
 */
fn main() {
    let list = vec![1, 2, 3];
    let mut iter = list.iter();
    let next_item = iter.next();
    // Perceba que precisamos declarar iter como mutável pois o método next muda seu estado. No
    // caso de um for isso não é necessário pois o for se apropria do iterator e torna-o mutável.

    // Iterator também possuem métodos definidos pela stdlib. Por exemplo, sum() e map():
    let sum = iter.sum();
    // map produz outro iterator:
   let sum2 = iter.map(|x| x + 1).sum();
}
