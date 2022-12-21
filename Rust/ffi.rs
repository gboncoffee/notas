/* 
 * Foreign functions são definidas dentro de extern blocks:
 */
extern "C" { // define a ABI.
    fn abs(input: i32) -> i32;
}
// São sempre unsafe
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
/*
 * Para tornar código de Rust chamável de outras linguagens, deve-se definir a ABI antes de fn e
 * marcar com #[no_mangle] para evitar que o compilador mude o nome da função.
 */
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
