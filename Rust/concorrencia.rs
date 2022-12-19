/*
 * Concorrência performática e segura é um dos objetivos de Rust. Tanto com concorrência de
 * mensagem quanto de estado compartilhado.
 */

use std::thread;
use std::time::Duration;

fn main() {
    // Spawnar threads é tão fácil quanto:
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // A call para thread::sleep força a thread a pausar, porém outras threads rodam nesse momento
    // a depender do scheduler do sistema operacional.
    //
    // Perceba que quando a main thread terminar, todas param também.

    /*
     * thread::spawn retorna um JoinHandle. Seu método .join(), que retorna um Result, trava a
     * thread atual até que a sua termine.
     */

    /*
     * Perceba que passar valores para uma nova thread não é tão simples. O compilador não tem como
     * saber quando a thread vai acabar, então mesmo passar referências imutáveis para serem
     * compartilhadas entre as duas threads é impossível pois elas podem acabar expirando antes da
     * nova thread acabar.
     *
     * Para passar a posse de um valor para a nova thread, usa-se "move" antes dos argumentos da
     * closure:
     */
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));
}

/*
 * Uma das maneiras de seguramente comunicar threads é a passagem de mensagens. Go faz isso
 * bastante, e um slogan de sua documentação diz tudo:
 *
 *     Não se comunique dividindo memória. Ao invés, divida memória se comunicando.
 * 
 * Para implementar isso, semelhante à Go, a Rust stdlib provém uma implementação de canais.
 *
 * Canais são monodirecionais, possuem transmissores e um ouvinte fixos. O canal é fechado se
 * qualquer um dos dois fechar a ligação. Perceba que, em Rust, canais podem ter múltiplos
 * transmissores. MPSC significa "Multiple producer, single consumer".
 */
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel(); // retorna um transmissor e um ouvinte

    // podemos mandar o transmissor para outra thread por exemplo
    thread::spawn(move || tx.send("Hello!").unwrap());
    // e receber uma mensagem dela
    let received = rx.recv().unwrap();
    println!("Other thread says: {received}");

    /*
     * Receivers tem dois métodos úteis: .recv() e .try_recv(). O primeiro vai bloquear a thread
     * até que uma mensagem esteja disponível. Assim que o valor é recebido, retorna Result<T, E>.
     * Se o canal for fechado, retorna Err.
     *
     * O segundo não bloqueia, recebe Ok(msg) caso haja uma mensagem e Err caso não exista. Dessa
     * maneira, pode-se criar um loop para manter a thread trabalhando enquanto não houverem
     * mensagens.
     *
     * Perceba que quando se passa valores através de mensagens, a thread que recebe-os também se
     * apossa deles.
     *
     * É possível utilizar rx como um iterator:
     */
    for received in rx {
        println!("Got {received}");
    }
}

/*
 * Para concorrência de memória compartilhada, Rust apresenta Mutex<T>. Exemplo:
 */
use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // retorna LockResult<MutexGuard>
        *num = 6;
    }
    println!("m: {:?}", m); // 6
}
/*
 * A call para .lock() trava a thread até que seja possível adquirir o lock da Mutex.
 *
 * Além disso, caso uma thread com o lock de panic! enquanto a sua call para .lock() está
 * esperando, ela retorna Err (Ah, sim, panic! é local a thread, não afeta o programa inteiro).
 *
 * Para se ter múltipla posse com múltiplas threads, como o que se faria com Rc<T> em um contexto
 * de thread única, NÃO SE USA Rc<T> pois não é thread-safe. O Arc<T> é sua versão thread safe, que
 * significa Atomic Reference Counter. Arc<T> tem a mesma API de Rc<T>.
 *
 * Note que Mutex<T> permite que ocorram deadlocks. Outra coisa que Rust não te impede de fazer.
 */

/*
 * A fins de curiosidade, note que Rust como linguagem tem poucas features de concorrência. As
 * anotadas até aqui são todas parte da stdlib e não da linguagem em si.
 */

/*
 * A trait Send marca tipos que podem ter posse transferida entre threads de maneira segura. Quase
 * todos os tipos da stdlib são, e qualquer tipo composto somente por tipos thread-safe também é.
 * Todos os primitivos exceto raw pointers são thread-safe. De maneira parecida, Sync marca tipos
 * que podem ser referenciados de diversas threads. Ou seja, &Send.
 */
