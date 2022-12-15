/*
 * Rust tem um sistema próprio de testes. Testar geralmente significa:
 *
 * 1 - Arrumar todos os dados e memória necessários.
 * 2 - Rodar o código que quer testar.
 * 3 - Verificar se o resultado é o esperado.
 *
 * Para criar funções de teste, deve-se adicionar o atributo #[test]. Ao rodar cargo run, essas
 * funções são todas compiladas em um binário de teste, que é rodado e o resultado de cada uma é
 * mostrado.
 *
 * Inclusive, os testes não são feitos somente para o código útil em si, mas também para qualquer
 * código de exemplo que esteja na documentação.
 *
 * O módulo que realiza os testes é um módulo normal que segue todas as regras de escopo e
 * namespaces.
 */

/*
 * Quando se inicia uma lib, o cargo te dá um lib.rs com um teste já:
 */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
/*
 * Perceba que testes falham quando algo dá panic, por isso usa-se assert_eq!.
 *
 * Outros macros úteis:
 */
panic!("pânico");
assert!(true);

/*
 * Eventualmente, precisa-se ter certeza que algo implica em panic!. Para isso, aplica-se
 * #[should_panic] à função. É possível adicionar uma cláusula para especificar como deve ser o
 * panic! para evitar passar panic! vindo de outras fontes:
 */
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test() {
        Vec::new()[8] = 1;
    }
}

/*
 * Também pode-se criar testes que retornam enums Result ao invés de darem panic!:
 */
#[cfg(test)]
mod tests {
    #[test]
    fn test() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal five"))
        }
    }
}
/* 
 * Isso facilita a criação de testes com o operador ?. Para verificar se algo retorna erro, porém,
 * usa-se assert!(value.is_err()). Não tem como usar #[should_panic] em testes que retornem Result.
 */

/*
 * Bandeiras para controlar a maneira como o cargo realiza os testes são passadas após o separador
 * --:
 *
 *   cargo test -- --help
 *
 * Testes com o cargo por padrão rodam paralelamente, e para tal não devem depender uns dos outros
 * ou de qualquer estado compartilhado. Para rodar testes em sequência, usa-se a bandeira
 * --test-threads=1
 *
 * Normalmente, o cargo esconde o output de testes que passam. Para mostrar, --show-output
 */

/*
 * Para rodar somente um teste, passa o nome da função ao cargo antes do separador --:
 *
 *   cargo test test_function_name
 *
 * Para rodar somente testes com certa substring no nome, mesma coisa:
 *
 *   cargo test add # roda todos os testes com add no nome
 *
 * É possível ainda ignorar testes com #[ignore], e eles só serão rodados quando passar --ignored
 * ou --include-ignored para o cargo.
 */

/*
 * A melhor prática é adicionar o módulo de testes unitários no arquivo com as funcionalidades a
 * serem testadas. Deve-se marcá-lo com #[cfg(test)] para que o compilador só os compile ao testar.
 *
 * Tests de integração, porém, são completamente externos às bibliotecas. São feitos em
 * tests/integration_test.rs. Não se marca o módulo com #[cfg(test)] pois o diretório inteiro é
 * tratado de maneira diferente. Perceba que cada arquivo lá é uma crate separada. Para rodar
 * somente testes de um arquivo específico em tests/, usa-se --test nome_do_arquivo.rs.
 *
 * Para criar código compartilhado entre testes em tests/, deve-se colocá-lo em um diretório
 * interno a tests/, como tests/common/. Isso pois arquivos em diretórios internos a tests/ não são
 * tratados como testes nem crates separadas.
 *
 * Perceba que crates binárias não podem ter testes de integração, somente unitários.
 */
