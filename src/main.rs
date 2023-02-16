// main.rs: this is a binary crate

use lumina_rust_graph::foo; // use a pub function from the lib crate
                            // makes stuff more convenient
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// binary crates have main method
fn main() {
    loop {
        let num = rand::thread_rng().gen_range(1..=100);

        println!("Guess the number {num}!");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        let variable = foo() + lumina_rust_graph::mymod::bar();
        println!("Guess {guess}, variable {variable}");
    }
}
