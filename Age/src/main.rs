use std::io;

fn main() {
    let current_year: i32 = 2025;

    // Input name
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim(); // remove newline

    // Input year of birth
    println!("Enter your year of birth: ");
    let mut year_of_birth_str = String::new();
    io::stdin()
        .read_line(&mut year_of_birth_str)
        .expect("Failed to read line");
    let year_of_birth: i32 = year_of_birth_str
        .trim()
        .parse()
        .expect("Please enter a valid number");

    if year_of_birth > current_year {
        println!("Year of birth cannot be in the future!");
        return;
    }

    let age: i32 = current_year - year_of_birth;

    println!("Hello, {}", name);
    println!("You are {} years old!", age);
}
