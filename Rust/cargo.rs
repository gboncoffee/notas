fn cargo() {

    /*
     * O Cargo é o build system de Rust. Utilizar só o rustc é possível, mas nós somos mais
     * tecnológicos que isso.
     *
     * Para criar um novo projeto com o Cargo, usa-se '$ cargo new <nome>'.
     *
     * Cargo usa um arquivo escrito em TOML (Tom's Obvious, Minimal Language) para configurar
     * projetos. É literalmente a coisa mais simples do mundo de ler e entender.
     *
     * O código em si do projeto deve estar em src. Organize-se.
     *
     * Após a primeira build, o cargo gerará um arquivo Cargo.lock que mantém informações sobre as
     * exatas dependências usadas. O arquivo é automaticamente gerenciado pelo cargo.
     *
     * Para buildar, '$ cargo build'. Para buildar e automaticamente rodar o binário resultante, '$
     * cargo run'.
     *
     * O Cargo também possui um comando útil para checar se o programa compila porém sem compilar,
     * '$ cargo check'.
     *
     * Para compilar o binário que o usuário final vai usar, o Cargo possui a bandeira '--release'.
     * Ela ativa certas otimizações que demoram mais para compilar, porém tornam o resultado final
     * mais (pasmem) otimizado.
     * 
     * Build comuns são colocadas em target/debug e releases em target/release.
     */
    
    /* 
     * É em gerenciar dependências que o cargo se destaca. Para adicionar dependências ao projeto,
     * coloque-as na seção 'dependencies' do Cargo.toml:
     *
     * [dependencies]
     * rand = "0.8.3"
     *
     * Perceba que o versionamento é importante. Por padrão, o Cargo vai considerar algo como
     * '0.8.3' como 'qualquer versão maior ou igual à 0.8.3 e menor que a 0.9.0'. Libs devem
     * atualizar o segundo número da versão quando mudarem a API, portanto uma mudança da 0.8.3
     * para a, ex, 0.8.4 não alterará significativamente o projeto, enquanto da 0.8 pra 0.9 pode
     * ter mudanças na API, ou seja, o código que você escreve deve ser mudado.
     *
     * Veja porém que o Cargo armazena a versão da lib utilizada na primeira build no Cargo.lock,
     * de maneira que, mesmo que essa lib atualize, por exemplo, da versão 0.8.3 para a 0.8.4, para
     * garantir que não haverão problemas, o Cargo mantém a lib na versão 0.8.3, exceto que você
     * mesmo atualize. Para atualizar para a última versão das libs, é só utilizar 'cargo update'.
     *
     * Perceba que, por causa da possível mudança da API, o update ignora versões cujo segundo
     * número subiu. Para atualizar, ex, da versão 0.8 para a 0.9, é necessário mudar a dependência
     * no Cargo.toml.
     */

    /*
     * O Cargo suporta profiles de compilação. Ao rodar 'cargo build', por padrão usa-se o profile
     * dev. Ao rodar 'cargo build --release', usa-se o profile release. Pode-se configurar cada
     * profile no Cargo.toml. Como exemplo, o padrão para o nível de otimização de código é esse:
     *
     * [profile.dev]
     * opt-level = 0
     *
     * [profile.release]
     * opt-level = 3
     */

    /*
     * O cargo tem capacidade para criar workspaces, que são conjuntos de pacotes que dividem o mesmo
     * Cargo.lock e diretório de output.
     *
     * Para tal, cria-se um diretório com um Cargo.toml especificando o caminho para cada pacote dentro
     * do diretório. Para cada um deles, cria-se uma nova crate com o cargo.
     *
     * Dentro de um mesmo workspace, dependências iguais devem ser da mesma versão. Para assegurar
     * isso, o Cargo cria somente um Cargo.lock no workspace.
     *
     * Quando em um workspace, para especificar qual binário deseja rodar, use `cargo run -p <crate>`.
     */
}
