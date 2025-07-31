// Demonstrating expressions (without explicit returns)
fn main() {
    let x = plus_one(999);
    println!("Value of x: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}