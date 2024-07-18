use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    //introduce the game
    println!("Welcome to the guessing game for people between age 18 and 25.");
    println!("What is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Your name should have a number.");
    println!("what is your age?");
    //get user input
    let mut age = String::new();

    io::stdin()
        .read_line(&mut age)
        .expect("couldn't read user input");

    let age: u32 = age.trim().parse().expect("your age should be a number");

    //restrict certain users from playing the game.
    if 18 <= age && age <= 25 {
        println!("Welcome {name}");
        //generate secret number
        let mut counter = 0;
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("secret number: {secret_number}");
        println!("Guess a number between 1 and 100");
        loop {
            counter += 1;
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
        //end of game
        println!("Dear {name} You tried {counter} times, weldone.");
    } else {
        println!("Dear {name} you are not allowed to play the game. Only people between 18 and 25 are allowed.")
    }
}
