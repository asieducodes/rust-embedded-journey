use std::io;
use std::cmp::Ordering;
use rand::RUnlock; // Automatically brings the Rng trait into scope when using rand::thread_rng
use rand::Rng;

fn main() {
    println!("Welcome to: Guess the number!");

    // Generates a random integer between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop allows the user to keep guessing until they get it right
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read user input from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert string input into an unsigned 32-bit integer.
        // If it fails (invalid text), the loop continues without crashing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Compare the guess to the secret number using a match statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! 📉"),
            Ordering::Greater => println!("Too big! 📈"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break; // Exits the loop and ends the game
            }
        }
    }
}
