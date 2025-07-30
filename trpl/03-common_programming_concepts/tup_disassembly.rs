// Demonstrating how to disassemble a declared tuple
// There are 2 ways
fn main() {
    let x: (f64, i32, u8) = (500.3, 234, 7);
    
    // Pattern matching
    let (p, q, r) = x;
    println!("p: {p}, q: {q}, r: {r}");
    
    // Using . operators
    let a = x.0;
    let b = x.1;
    let c = x.2;
    println!("a: {a}, b: {b}, c: {c}");
}