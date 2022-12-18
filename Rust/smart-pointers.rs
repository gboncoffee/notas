/*
 * Referências são como pointers comuns: Não possuem overhead nem outras capacidades especiais.
 *
 * Smart pointers, ao contrário, são estruturas de dados que funcionam como pointers porém
 * também possuem metadados e outras capacidades. Foram originados em C++ e existem em
 * diversas linguagens.
 *
 * Rust possui vários tipos de smart pointers, como por exemplo um que tem funcionalidade
 * semelhante a um garbage collector, porém somente para si próprio: o contador de referências,
 * que mantém registro do número de referências a um dado e quando não há mais nenhuma, é
 * eliminado.
 *
 * Enquanto referências emprestam dados, smart pointers costumam ser donos dos dados que
 * apontam.
 *
 * Entre exemplos de smart pointers na stdlib, há String e Vec<T>.
 *
 * Geralmente implementa-se smart pointers com structs com as traits Deref e Drop. Deref
 * permite que sua struct se comporte como uma referência e Drop customiza o código que roda
 * quando uma instância da struct sai de escôpo.
 */

/*
 * O smart pointer mais direto é Box<T>. Ela aloca seu dado na heap e mantém um ponteiro na
 * stack. Nenhum overhead além de guardar os dados na heap. Seus casos de uso normalmente são:
 * - Quando se tem um tipo cujo tamanho não se sabe em compile-time e quer-se usar dele em um
 * contexto de tamanho fixo.
 * - Quando se tem muitos dados para transferir a posse e deve-se ter certeza de que eles não
 * serão copiados.
 * - Quando se quer ser dono de um valor que implemente certa trait, porém não importa seu tipo
 * específico.
 *
 * Como exemplo, tem-se os tipos recursivos (como nós de árvores). Como podem ser aninhados
 * indefinidamente, o compilador não tem como saber o tamanho em compile-time.
 */

/*
 * Para definir smart pointers que sejam dereferenciáveis, usa-se a trait Deref e o método
 * obrigatório deref:
 */
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T; // define um tipo associado à trait
    fn deref(&self) -> &Self::Target {
        &self.0 // o método deve retornar um ponteiro para o valor que quer dereferenciar.
    }
}
// Da mesma maneira, pode-se usar DerefMut para referências mutáveis.

/*
 * Rust implementa deref coercion. Basicamente, enquanto tipos implementarem Deref, o compilador
 * continuará dereferenciando-os até chegar no tipo certo. Significa que um ponteiro para um
 * ponteiro para um ponteiro para [...] pode ser dereferenciado com um só *. Tudo calculado em
 * compile-time.
 */

/*
 * A trait Drop implementa um método drop(&mut self) que é chamado quando a instância estiver para
 * ser eliminada. Não é possível chamar esse método diretamente, pois é um destructor. Para
 * eliminar uma instância antes de Rust fazer isso automaticamente, deve-se chamar std::mem::drop.
 */

/*
 * Rc<T> é um smart pointer com contador de referências. É a maneira de se ter um valor com
 * múltiplos donos. Quando o contador zera, o valor é eliminado. É usado quando precisa-se de um
 * valor que será alocado e lido por diversas partes do programa, sem ter como saber em
 * compile-time quando que a última deixará de usá-lo.
 *
 * Serve somente para cenários de thread única.
 *
 * Veja o exemplo com listas funcionais:
 */
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // a é uma instância de Rc<List>
    let b = Cons(3, Rc::clone(&a)); // b e c são instâncias de List::Cons(i32, Rc<List>) 
    let c = Cons(4, Rc::clone(&a)); // ambas usam Rc::clone para serem donas de a e incrementarem
}                                   // seu reference counter.
/*
 * Perceba que com Rc<T>, a convenção é usar Rc::clone ao invés do método .clone() para diferenciar
 * dos métodos .clone() de outros tipos que fazem uma cópia dos dados. Rc::clone não copia nada,
 * somente incrementa o reference counter do objeto passado referenciado nos argumentos.
 */

/*
 * RefCell<T> é um wrapper semelhante à Box<T>, porém serve para assegurar em runtime que as regras
 * de empréstimo estão sendo seguidas. Caso contrário, o programa dá panic!. Serve para que certas
 * coisas seguras que o compilador reclamaria por ser conservador sejam permitidas.
 *
 * O compilador rejeita programas que ele não tem CERTEZA de que são seguros, por assim dizer. Por
 * isso é conservador. O compilador faz análise de programas, e se não consegue analisar um
 * programa assim como no Halting Problem, vai rejeitá-lo.
 *
 * Dessa maneira, RefCell<T> serve para casos em que se está seguro da segurança do programa porém
 * o compilador não. Só serve para single-thread também.
 *
 * Como RefCell<T> permite a checagem de empréstimos mutáveis em runtime, pode-se mudar o valor de
 * uma RefCell<T> mesmo se ela for imutável.
 *
 * Para emprestar o que a RefCell<T> contém, usa-se .borrow(), e para emprestar mutavelmente,
 * .borrow_mut(). Da mesma maneira que o compilador, porém implementado em runtime, RefCell<T>
 * permite somente um empréstimo mutável por vez. O problema é que, como isso é verificado em
 * runtime, o programa dará panic! caso tal regra seja quebrada.
 *
 * Para se ter mais de um empréstimo mutável de uma vez, então, usa-se Rc<RefCell<T>>.
 */
