use std::fs;
// std::fs possui o básico para se lidar com arquivos
fn main() {
    let path = "foo.txt";

    /*
     * entre as funcionalidades interessantes há:
     */

    // ler para Result<String>
    let content = fs::read_as_string(path)
        .expected("Cannot read file!")
}
