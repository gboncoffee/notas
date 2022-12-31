use std::thread

/*
 * Entre as features de programação funcional que Rust implementa, podemos destacar closures e
 * iterators
 */

fn main() {
    /*
     * Closures são as funções de primeira-classe em Rust, que podem ser evaluadas em contextos
     * diferentes e recebem o ambiente em que foram definidas.
     */

    /*
     * Perceba que estão sujeitas às regras do borrow-checker assim como qualquer função. Dessa
     * maneira, isso é válido:
     */
    let list = vec![1, 2, 3];
    println!("Antes de definir a closure: {:?}", list);

    let borrows = || println!("Na closure: {:?}", list);

    println!("Antes de chamar a closure: {:?}", list);
    borrows();
    println!("Após chamar a closure: {:?}", list);

    /*
     * Porém, caso precisa-se modificar o vetor, será necessário o empréstimo mutável:
     */

    let mut list = vec![1, 2, 3];
    println!("Antes de definir a closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // essa call não é mais permitida pois list foi emprestada modificavelmente para a closure.
    // println!("Antes de chamar a closure: {:?}", list);
    borrows_mutably();
    // não há mais nenhuma call para a closure, portanto pode-se emprestar list imutavelmente.
    println!("Após chamar a closure: {:?}", list);

    /*
     * Para forçar a closure a tomar posse de coisas, usa-se a keyword move, bastante utilizada
     * quando se precisa passar dados de uma thread a outra:
     */
    thread::spawn(move || println!("Na thread: {:?}", list))
        .join()
        .unwrap();

}
/*
 * Para retornar closures, deve-se retornar um ponteiro:
 */
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
