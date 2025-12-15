use rand::Rng;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    println!("Guess the number between 1 and 100:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let guess: u32 = match input.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please enter a number");
        continue;
    }
};

if guess < secret {
    println!("Too small!");
} else if guess > secret {
    println!("Too big!");
} else {
    println!("You win!");
    break;
}
    }
}