use std::env;

fn main() {

    /*
     * Em std::env, a função args() é um iterator sobre os argumentos de linha de comando do
     * programa. Ele trabalha só com unicode válido e dá panic! se receber unicode inválido. Para
     * usar argumentos com unicode inválido, usa-se args_os(), que retorna OsString e não String.
     */
    let args: Vec<String> = env::args().collect();

    /*
     * Para ler envvars:
     *
     * env::var() retorna um Result. Caso a váriavel não exista, é Err
     */
    let var_exists = env::var("VAR").is_ok();
}
