//
// Generics são um meio de criarmos funções, enums e estruturas que possam ser utilizadas com
// vários tipos de dados diferentes.
//

// 
// Usa-se um nome genérico para definir o dado recebido. Geralmente, T.
//
// Para declarar uma função que recebe um generic T e retorna-o:
fn func<T>(arg: T) -> T {

    arg
}
// Em enums:
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// Em structs:
struct Point<T> {
    x: T,
    y: T,
}
// Note que se quiser usar tipos diferentes, precisa de um para cada:
struct Point<T, U> {
    x: T,
    y: U,
}
// Em métodos:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Note ainda que os tipos genéricos aplicados aos parâmetros de métodos não são necessariamente os
// mesmos dos definidos para a struct em si.
//
// Note que ainda é possível fazer métodos que só se aplicam para certos tipos:
impl<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//
// Generics não diminuem a performance de Rust em runtime porque em compile time o compilador
// identifica quais tipos serão usados e gera código diferente para cada um. Performancesegurança.
//
// ;)
