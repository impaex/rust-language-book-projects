use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // The `gen_range` function generates a random number in the specified range
    // The `1..=100` syntax creates a range from 1 to 100, following the start..=end format for inclusive ranges
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Variables are immutable by default
        // Using `mut` makes them mutable
        // The :: syntax indicates a function or associated item of a type
        // In this example, `new()` is a function of the `String` type
        let mut guess = String::new();

        io::stdin()
            // & is indicating a reference to the variable, which allows multiple parts of the code to
            //       access the same data without needing to copy it into memory multiple times
            .read_line(&mut guess)
            // Readline returns a Result type, which is an enum that can be either Ok or Err
            // The expect method is called on the Result type to handle the case where reading fails
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catch-all pattern that matches any value
            Err(_) => continue,
        };

        // {} are placeholders for variables in the string
        // They can contain a variable name, to fill in an expression, do the following:
        // println!("y + 2 = {}", y + 2);
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
