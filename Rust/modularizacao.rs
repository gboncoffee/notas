// 
// A modularização em Rust é feita dividida em várias partes:
//
// - Packages/Pacotes:  pacotes do Cargo;
// - Crates:            árvore de módulos que produz uma lib ou binário;
// - Modules/Módulos:   controle do escopo;
// - Path:              maneira de nomear itens.
//

// 
// Pacotes são uma ou mais crates que provem uma funcionalidade. Um pacote 
// possui um arquivo Cargo.toml que descreve como fazer a build dessas 
// crates.
//
// Crates podem ser libs ou binárias. Libs não compilam para binários e nem
// possuem função main.
//
// Pacotes precisam conter pelo menos uma crate, sendo qualquer número de
// binárias e até uma única lib.
//
// Perceba que o Cargo, no Cargo.toml, não exige menção à src/main.rs como
// root de uma crate binária, pois assume que esse arquivo é a root de uma
// crate binária com o mesmo nome do pacote. A mesma coisa para libs com
// src/lib.rs.
//
// Cada arquivo que for colocado em src/bin/ será uma nova crate binária.
//

// 
// O compilador sempre começa pela root da crate, geralmente src/lib.rs ou
// src/main.rs. Na root, pode-se declarar módulos:
mod garden;
// o compilador irá procurar o módulo garden diretamente se declarado com {}
// e não ;, depois no arquivo src/garden.rs, depois em src/garden/mod.rs.
//
// Módulos podem declarar submódulos. Caso garden declare o submódulo
// vegetables, o compilador procurará diretamente, depois em 
// src/garden/vegetables.rs e depois em src/garden/vegetables/mod.rs.
//
// Se o submódulo estiver público, a partir do momento em que é incluído na
// compilação, pode ser acessado de qualquer lugar de uma crate com o path
// crate::garden::vegetables.
//
// Por padrão, módulos são privados a seus parentes. Para torná-los públicos,
// deve-se declarar pub mod. Seus itens continuam privados exceto que
// declarados com pub também.
pub mod garden;
// No caso de structs, há um porém: a struct ser pública não torna seus 
// fields públicos. No caso de enum, ela ser pública torna todas as variações
// públicas.

// Em um escopo, use cria atalhos para reduzir a repetição dos namespaces:
use crate::garden::vegetables::Asparagus;
// permite usar o item Asparagus só:
Asparagus::new();
// Se for trazer dois nomes iguais ao escopo, pode renomear um deles:
use std::fmt::Result;
use std::io::Result as IoResult;

// 
// Paths de módulos podem ser absolutos, referenciando uma crate ou usando
// a palavra crate para referenciar a própria crate ou relativos, que usam
// self, super ou identificadores no módulo atual.
//
// Tanto paths absolutos quanto relativos tem o separador ::
//
// É possível criar paths relativos ao módulo parente usando a palavra super
//
// Usar pub use possibilita que código que chame o seu escopo use o seu use.
// Ex: em uma crate restaurant, isso:
pub use crate::front_of_house::hosting;
// faz com que o usuário da API possa chamar por funções de hosting assim:
restaurant::hosting::function();
// É possível ainda declarar uses aninhados. Se for usar std::io e 
// std::io::Write, pode escrever assim por exemplo:
use std::io::{self, Write}

// 
// Em pacotes que possuem tanto uma crate lib e binárias, a árvore de módulos
// deve ser definida em src/lib.rs e dessa forma, as crates binárias acessam
// a lib exatamente da mesma maneira que qualquer outra crate acessaria.
//
