use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut rounds = 0;
    loop {
        let secret = rand::rng().random_range(1..=100);

        println!("\nGuess the number between 1 and 100 (or type 'exit' to quit):");

        let mut guesses = 0;
        loop {
            guesses += 1;
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input_str = input.trim();
            
            if input_str == "exit" {
                println!("Goodbye!");
                println!("You played {} games!", rounds);
                return;
            }

            let guess: u32 = match input_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            continue;
        }
    };

            match guess.cmp(&secret) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    println!("It took you {} guesses!", guesses);
                    rounds += 1;
                    break;
                }
            }
        }
    }
}