/*
 * Static Code Analysis é conservadora por natureza. Sempre que o compilador encontrar código que
 * não tem certeza da segurança, irá rejeitar. Unsafe é a maneira de dizer ao compilador "Tudo bem,
 * eu já sou um menino grande e sei exatamente o que estou a fazer.".
 *
 * Além disso, hardware em si é inseguro. Para lidar diretamente com o hardware, ou com o sistema
 * operacional, ou até escrever seu próprio kernel, precisa aceitar a responsabilidade de já ser
 * bem crescidinho e usar Unsafe Rust.
 */
fn main() {
    /*
     * Para trocar para Unsafe Rust, use a keyword unsafe e inicie o unsafe block:
     */
    unsafe {
        /*
         * Veja que unsafe não é festa da uva: há apenas 5 superpoderes que se tem dentro de um
         * bloco inseguro. O resto continua a funcionar normalmente:
         *
         * - Dereferenciar um ponteiro nulo;
         * - Chamar uma função ou método inseguro;
         * - Acessar ou modificar uma váriavel estática mutável;
         * - Implementar uma trait unsafe;
         * - Acessar fields de unions.
         * 
         * Perceba que Unsafe não desliga o borrow checker.
         */

        /*
         * Dereferenciar ponteiros nulos:
         *
         * Unsafe trás um novo tipo: raw pointer. Similar a ponteiros em C:
         * - Podem ignorar as regras de empréstimo tendo ponteiros mutáveis e imutáveis, ou vários
         * ponteiros mutáveis;
         * - Não tem garantia de apontar para memória válida;
         * - Podem ser nulos;
         * - Não implementam nenhuma limpeza automática.
         *
         * É possível criar raw pointers em código seguro, porém dereferenciá-los deve ser feito em
         * unsafe.
         */
        let mut num = 5;
        let r1 = &num as *const i32;
        let r1 = &mut num as *mut i32;
    }
}

/*
 * Cria-se unsafe functions de maneira semelhante. Precisa de unsafe para chamá-las.
 */
unsafe fn do_nothing() {}
fn main() {
    unsafe {
        do_nothing();
    }
}
/*
 * Cria-se unsafe traits semelhantemente:
 */
unsafe trait Foo {
    // métodos
}
unsafe impl Foo for i32 {
    // implementações
}
