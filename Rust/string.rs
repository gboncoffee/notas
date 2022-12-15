fn main() {

    // O tipo String representa (pasmem) strings em Rust.

    /*
     * Uma String que não seja um literal (string hard-coded) é, obviamente, salva na heap. Na
     * stack, há um ponteiro para a string na heap, um valor de capacidade e um de tamanho.
     *
     * Por isso, Strings não devem ser tratadas como tipos comuns. Ex:
     */
    let mut x = 5; // aqui, o valor 5 é copiado para a outra váriavel
    let mut y = x; // de forma que temos uma cópia dele

    let mut s = String::from("hello"); // aqui, somente os valores da stack são copiados, de maneira
    let mut t = s;                     // que temos só um texto na heap

    /*
     * Métodos úteis:
     */
    String::new();                 // cria uma string na heap, manejada pela linguagem
    String::from("literal");       // cria uma string na heap iniciada pelo argumento literal passado
                                   //
    string.push_str("new text");   // append para a string (deve ser mutável)
    string.trim(); // remove espaços em branco e \n ao redor da string.
    string.parse(); // transforma strings em diferentes tipos de números.

    //
    // Strings são parte das collections da stdlib. São uma coleção de dados
    // UTF-8. Por serem unicode, podem guardar muitos caractéres diferentes:
    let hello = String::from("Hello");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("السلام عليكم");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("你好");

    //
    // Assim como vetores, podem crescer pushando data para elas, usando + ou
    // format!. Para pushar um slice para a string, usa-se push_str:
    let mut s = String::from("foo");
    s.push_str("bar");
    // ou o método push para pushar só um caractér:
    let mut s = String::from("lo");
    s.push('l');

    // 
    // Usando o operador +, a primeira string é apossada e a segunda precisa ser
    // emprestada, portanto temos algo assim:
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s1 = s1 + &s2; // perceba que s1 sai de escopo, portanto para continuar
                       // em uso precisa ser redefinida
    let mut s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    s1 = s1 + &s2; // ou pode-se declarar com mut e reusá-la. Note que o +
                   // "soma" uma String com um &str (ou &String, transformando
                   // em &str[..]). s2 continua a ser uma String válida após.
    // 
    // Para evitar problemas de posse e manter um código mais bonito, o macro
    // format!, que só usa referências, pode ser usado:
    let s1 = String::from("tic");
    let s1 = String::from("tac");
    let s1 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    //
    // Strings não podem ser indexadas normalmente. Não é possível acessar 
    // caractéres individuais com s[num] porque Strings são encodadas em UTF-8. 
    // Não se usa strings em Rust como se usa em C, em que são simplesmente um 
    // array de chars.
    //
    // Strings em Rust são basicamente wrappers de Vec<8>, porém mais de um só
    // item desse vetor pode ser uma letra só. Exemplo:
    //
    // A palavra hindi नमस्ते tem 18 bytes. Porém, são 6 caractéres Unicode (dois
    // são acentos). O que uma pessoa diria é que essa palavra tem só 4 letras.
    //
    // Seria possível indexar Strings se contassemos tudo desde o começo, porém
    // indexação é uma operação que se espera que leve tempo O(1), então Rust
    // também não a faz por padrão.
    //

    //
    // A PIOR maneira de indexar Strings em Rust é usando slices para pegar
    // um range de bytes da String. Porém, caso tente quebrar um char Unicode
    // no meio, o programa irá dar panic.
    //

    //
    // Para iterar uma String, considerando cada char Unicode, deve-se usar o
    // método String.char(), e para iterar considerando cada byte, deve-se usar
    // o método String.bytes():
    for c in "नमस्ते".chars() {} // 6 vezes
    for b in "नमस्ते".bytes() {} // 18 vezes
}
