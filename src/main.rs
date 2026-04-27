use std::io;
use rand;

fn main() {
    println!("---------- Guess the number! ----------");
    let target = rand::random_range(1..=100);
    println!("Target: {}", target);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        let ip = guess.trim().parse::<i32>().unwrap(); 

        if target == ip {
            println!("You guessed: {guess}");
            return;
        } 
        else if target > ip{
            println!("Too low! Try higher.\n");
        }
        else {
            println!("Too high! Try lower.\n");
        }
    }
}