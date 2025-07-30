// Demonstrating Array Index Out-of-Bounds exception for incorrect input
use std::io;

fn main() {
    let a = [1, 3, 5, 7, 9];

    println!("Please enter an array index: ");
    let mut index = String::new();

    // Capture input
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // Check validity of the input (is it a number?)
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value at index {index} is: {element}");
}