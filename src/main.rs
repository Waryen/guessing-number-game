use std::io;
use rand::Rng;
use colored::*;

fn main() {
    let number_to_find = rand::thread_rng()
        .gen_range(1..100);
    let mut guess = String::new();

    println!("Guess a number:");

    // FIXME programm panick after 2 guess

    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = guess.trim()
            .parse().expect("Type in a number");

        if guess < number_to_find {
            println!("{}", "Too small!".red());
        } else if guess > number_to_find {
            println!("{}", "Too big!".red());
        } else {
            println!("{}", "You win!".green());
            break;
        }
    }
}
