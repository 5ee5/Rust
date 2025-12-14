use rand::Rng;
use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let start: f64 = read_input("Enter the start point:")
        .parse()
        .expect("Invalid number");

    let end: f64 = read_input("Enter the end point:")
        .parse()
        .expect("Invalid number");

    let choice = read_input(
        "Do you want a whole number or a floating-point number? (whole/float)"
    );

    let mut rng = rand::thread_rng();

    match choice.as_str() {
        "whole" => {
            let n = rng.gen_range(start as i64..=end as i64);
            println!("Random number: {}", n);
        }
        "float" => {
            let n = rng.gen_range(start..end);
            println!("Random number: {}", n);
        }
        _ => println!("Invalid choice"),
    }
}
