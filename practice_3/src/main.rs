use std::io;

fn main() {

// PRACTICE 1: Convert temperatures between Fahrenheit and Celsius.
    println!("Please type a number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: f64 = number.trim().parse().expect("Please type a float number!");

    let celcius_temp = fahrenheit_to_celcius(number);
    println!("Fahrenheit temperature of given number is {celcius_temp}!!");
    let fahrenheit_temp = celcius_to_fahrenheit(number);
    println!("Celcius temperature of given number is {fahrenheit_temp}!!");
    println!();

// PRACTICE 2: Generate the nth Fibonacci number.
    println!("Please type a number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please type an integer!");
    
    let fibonacci_number = nth_fibonacci(number);
    println!("{}th Fibonacci number is {}!", number, fibonacci_number);
    println!();


// PRACTICE 3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
            // taking advantage of the repetition in the song.

    let lyrics = ["And partridge in a pear tree", "Two turtledoves", "Three French hens", "Four calling birds", 
    "Five gold rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing" ,
    "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    
    let ordinal_lists = ["first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let ending = String::from("A partridge in a pear tree");

    let mut idx = 0;    
    for ordinal in ordinal_lists {
        println!("On the {ordinal} day of Christmas, my true love sent to me");
        
        for number in (0..idx+1).rev() {
            if idx == 0 {
                println!("{ending}");
                continue;
            }
            println!("{}", lyrics[number]);
        }
        println!();
        idx += 1;
    }   println!("{}", lyrics[0]);
}

fn fahrenheit_to_celcius(fahrenheit_temp: f64) -> f64 {
    let celcius_temp:f64 = (fahrenheit_temp-32.0) / 1.8;
    celcius_temp
}

fn celcius_to_fahrenheit(celcius_temp: f64) -> f64 {
    let fahrenheit_temp:f64 = celcius_temp*1.8 + 32.0;
    fahrenheit_temp
}

fn nth_fibonacci(mut number: i32) -> i32 {
    let mut first = 1; let mut second = 1; let mut result = 1;
    while number>1 {
        result = first + second;
        first = second; second = result;
        number -= 1;
    }
    result
}