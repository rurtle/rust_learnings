// Demonstrating the usage of function
// Unlike C/C++, other functions can be declared anywhere in the file
// without having to worry about the placement of the function
fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
}

fn another_function(qty: i32, label: char) {
    println!("Another function!");
    println!("Passing {qty} {label} to another_function()");
}