use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1;
        {let x = x * 2; println!("The value of x in the inner scope is: {x}");}
        {let x = x * 3; println!("The value of x in the inner scope is: {x}");}
        println!("The value of x is: {x}");

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = 5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
        println!("Sum: {sum}");
        println!("Difference: {difference}");
        println!("Product: {product}");
        println!("Quotient: {quotient}");
        println!("Truncated: {truncated}");
        println!("Remainder: {remainder}");

    let heart_eyed_cat = '😻';
        println!("{heart_eyed_cat} can be a character too");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
        println!("The value of y is: {y}");

    let empty = ();

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
              let first = months[0]; let second = months[1];
              println!("{first} {second}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
