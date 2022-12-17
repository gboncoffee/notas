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
