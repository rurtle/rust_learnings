// Demonstrating if expressions
fn main() {
    // Let's check basic if expressions first
    let mut x = 3;
    if x < 5 {
        println!("x is less than 5");
    } else {
        println!("x is greater than 5");
    }

    // Slightly more complex - if else here
    x = 6;
    if x % 4 == 0 {
        println!("{x} is divisible by 4");
    } else if x % 3 == 0 {
        println!("{x} is divisible by 3");
    } else if x % 2 == 0 {
        println!("{x} is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3 or 2");
    }

    // Finally, if with let
    let condition = true;
    let number = if condition {233} else {112};
    println!("Value of number: {number}");
}