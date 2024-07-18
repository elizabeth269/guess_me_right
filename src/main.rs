use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    //introduce the game
    println!("Welcome to the guessing game for people between age 18 and 25.");

    //get user input
    let mut name = String::new();
    let mut age = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Your name should have a number.");
    io::stdin()
        .read_line(&mut age)
        .expect("couldn't read user input");

    let age: u32 = age.trim().parse().expect("your age should be a number");

    //restrict certain users from playing the game.
    if 18 > age && age < 25 {
        println!("Welcome{name}");
        //generate secret number
        loop {
            let secret_number = rand::thread_rng().gen_range(1..=100);
            println!("secret number: {secret_number}");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("couldn't read the message");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too small"),
                Ordering::Greater => println!("too big"),
                Ordering::Equal => {
                    println!("you win");
                    break;
                }
            }
        }
    } else {
        println!("Dear {name}, you are not allowed to play the game. Only people between 18 and 25 are allowed.")
    }
}
