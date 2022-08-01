use std::io; //import io from standard library
use std::io::Write; //import print stuff
use rand::Rng; //import random

fn main() { //define the function main
    println!("Guess the number!"); //print

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number from 1 to 100

    println!("The secret number is: {secret_number}"); //print secret number

    print!("Please input your guess. "); //another print
    io::stdout().flush().expect("Failed to flush"); //dont add new line

    let mut guess = String::new(); //make a new mutable string

    io::stdin()                         //get user input
        .read_line(&mut guess)          //store in guess variable
        .expect("Failed to read line"); //send error message

    println!("You guessed: {guess}"); //prints user guess
}
