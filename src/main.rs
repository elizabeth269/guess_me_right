use std::io;
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
}
