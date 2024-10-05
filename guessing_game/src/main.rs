use std::io;

fn main() {
    println!("Welcome to guessing game");
    println!("Enter a number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error reading");

    println!("Your guess is: {guess}");
}
