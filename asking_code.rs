use std::io::{self, Write};

fn main() {
    // Prompt for the user's name
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // Ensure the prompt appears before input
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim(); // Remove any trailing newline

    // Prompt for the user's age
    let mut age_str = String::new();
    print!("Enter your age: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut age_str)
        .expect("Failed to read line");
    let age: u32 = age_str
        .trim()
        .parse()
        .expect("Please enter a valid number for age");

    // Output the greeting
    println!("Hello {}, you are {} years old.", name, age);
}