fn main() {
    /*
     * Patterns em Rust são:
     *
     * - Literals;
     * - Arrays, enums, structs ou tuplas deestruturadas;
     * - Variáveis;
     * - Wildcards;
     * - Placeholders.
     */
     
    // Patterns são válidos em match arms:
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }

    // Em if let:
    if let Some(color) = favorite_color {
        println!("Using {color}: it's your favorite color!");
    } else if let Ok(age) = age {
        if age > 30 { // perceba que age é uma shadowed variable e só é válida dentro do bloco
            println!("Using purple");
        } else {
            println!("Using green");
        }
    } else {
        println!("Using black");
    }

    // Ao usar match, o compilador checa exaustividade (todos os padrões tem uma correspondência).
    // Com if let isso não ocorre.

    // Em while let:
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // Em for:
    for (index, value) in vec![3, 2, 1].iter().enumerate() {
        println!("position: {index} ; value: {value}");
    }

    // Na verdade, todo let é um pattern:
    let PATTERN = EXPRESSION;
    // Tanto que isso é válido, pois é só pattern matching:
    let (x, y, z) = (1, 2, 3);
}
// Em parâmetros de funções:
fn recursive_tuple(&(x, y): &(i32, i32)) {
    let value = (1, 2)
    recursive_tuple(&value);
}

/*
 * Patterns são refutáveis (o match pode falhar) ou irrefutáveis (o match nunca falha). Um simples
 * variable assign é irrefutável: É impossível que o nome x não dê match com 1:
 */
let x = 1;

/*
 * É possível dar match em múltiplos patterns (ué, virou regex?):
 */
match x {
    1 | 2 | 3 => println!("One, two or three"),
    _ => println!("Other number"),
}
/*
 * Nesse caso, aliás, seria melhor usar um range inclusivo:
 */
match x {
    1..=3 => println!("One, two or three"),
    _ => println!("Other number"),
}
// Ranges são permitidas tanto com números quanto chars.

/*
 * Patterns também podem ser usados para quebrar tuplas, structs, etc:
 */
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: x, y: y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    // Porém isso é melhor escrito assim:
    let Point { x, y } = p;
    // Útil também com match:
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

/*
 * .. expande para cobrir valores não explicitamente listados:
 */
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    /*
     * É possível usar if em um match para representar padrões mais complicados:
     */
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => println!("Number? Which number?"),
    }
}

/*
 * O operator "at" @ permite criar uma váriavel ao mesmo tempo que testa um valor:
 */
enum Message {
    Hello { id: i32 },
}
fn main() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => println!("Integer between 3 and 7: {id_var}"),
        _ => return,
    }
}
// Sem o operador dando bind em id_var, não teria como utilizar o valor diretamente numa váriavel.
