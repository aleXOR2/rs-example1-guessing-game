#![allow(non_snake_case)]

use std::io;

fn main() {
    let apples = 6; // immutable
    let bananas = 5; // mutable

    let mut guess = String::new(); // mutable

    println!("Guess the number game!");
    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    let guessToInt = strip_trailing_newline(&guess).parse::<i32>().unwrap();
    let conditional_apples_res = guessToInt == apples;
    let conditional_bananas_res = guessToInt == bananas;
    println!("you guessed apples: {}", conditional_apples_res);
    println!("you guessed bananas: {}", conditional_bananas_res);
}


fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or_else(|| input)
}

