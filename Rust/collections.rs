//
// Rust possui um punhado de estruturas de dados compostas na stdlib.
//
// Arrays e tuplas não estão inclusos, coleções são estruturas salvas na
// heap, então sem tamanho sabido em compile-time.
//

// 
// Vetores 
// Vec<T>
//

//
// Para criar um novo vetor, lembre-se de especificar o tipo:
let v: V<isize> = Vec::new();
// Para criar um vetor com valores iniciais, há um macro:
let v = vec![1, 2, 3]
// O método Vec<T>.push() adiciona elementos (lembre-se que o vetor
// precisa ser mut)
let mut v = Vec::new(); // sem tipo porque depois vamos pushar valores
v.push(4);              // então o compilador entende depois.

//
// Rust provém dois meios de acessar valores em vetores, por referências
// ou pelo método .get():
let v = vec![1, 2, 3, 4, 5];
let third: &isize = &v[2];
println!("The third element is {}", third);
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("No third element"),
}
// Perceba que uma tentativa de referência a um elemento inexistente
// causa panic, mas o método .get() retorna um enum.
//
// Perceba ainda que se você tiver uma referência a um elemento do 
// vetor, para o compilador, é como se tivesse uma referência ao vetor
// inteiro e o borrow checker vai reclamar se tentar fazer um borrow
// mutável novamente (que inclui Vec<T>.push()).
//

// 
// Para iterar em vetores, for:
let v = vec![1, 2, 3, 4];
for i in &v {
    println!("{}", i);
}
// porém se for modificar os itens deve emprestar o vetor como mutável:
let mut v = vec![1, 2, 3, 4];
for i in &mut v {
    *i += 10; // deve-se dereferenciar com *
}

// 
// Hash maps
// HashMap<K, V>
//

//
// Rust tem minha estrutura de dados favorita por padrão, a hash table! Também
// conhecida na noite como dicionário, hash map, map, associative array, etc.
//
// HashMap<K, V> mapeia chaves do tipo K para valores do tipo V. Não está
// incluso por padrão, precisa incluir com use.
//
// Para criar, só usar new(), e inserir com insert():
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Pink"), 50);
scores.insert(String::from("Purple"), 30);
// Note que insert() sobreescreve valores de chaves existentes, caso tente-se
// inserir uma chave existente.
//

// 
// Perceba que valores que não são copiáveis são apossados pelo hashmap, mas
// valores copiáveis não são. Se adicionar uma referência ao hashmap, ele não se
// apossará do valor, porém a referência precisa ser válida enquanto o hasmap
// for.
//
// Pode-se pegar um valor do hasmap com o método .get(), que retorna uma Option,
// sendo None se não houver tal chave registrada.
//

// 
// O HashMap é iterável:
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

//
// O método .entry() retorna um enum Entry que representa se existe ou não uma
// chave no hashmap. O método .or_insert() desse enum insere algo no hashmap
// associado a ela caso não haja nada. Portanto, inserir chaves somente se essas
// não existirem é tão fácil quanto:
scores.entry(String::from("Yellow")).or_insert(50);
// O método .or_insert() retorna uma referência mutável para o valor da chave
// caso ela já exista. Dessa maneira, também podemos utilizá-lo para mudar tal
// valor. Exemplo com um contador de palavras:
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

//
// Por padrão, HashMap usa o algoritmo SipHash, que é resistente a ataques de
// DoS envolvendo hashmaps. Não é o mais rápido que existe, e caso queira usar 
// outro, pode criar um Hasher implementando a trait BuildHasher, ou usar um
// de alguma lib do crates.io.

}
