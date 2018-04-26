extern crate rand;

use std::io; // brings io library into scope
use std::cmp::Ordering; // Ordering is an enum like Result, but variants are Less, Greater, Equal
use rand::Rng; // Rng is a trait

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // variables are immutable by default

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut makes this mutable, String::new() returns a new instance of string
        // :: indicates new is an associated function of String (~static method)

        io::stdin().read_line(&mut guess) // &mut is a mutable reference to guess?
            .expect("Failed to read line"); // takes the argument given from io::Result (Ok, Err)

        let guess: u32 = match guess.trim().parse() { // we already had guess but use shadowing here
            Ok(num) => num, // on success, return Ok containing number in guess
            Err(_) => continue, // on fail, _ is catchall value, will start loop again (continue)
        };
        // u32 is good choice for small +ve number, rust will infer secret_number should be u32 as well

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // cmp compares two values (comparing guess and secret_number)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
