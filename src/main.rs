use std::io; //import io from standard library
use std::io::Write; //import print stuff
use rand::Rng; //import random
use std::cmp::Ordering; //import ordering things

fn main() { //define the function main
    println!("Guess the number!"); //print

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number from 1 to 100

    println!("The secret number is: {secret_number}"); //print secret number

    loop { //make a new while true loop
        print!("Please input your guess. "); //another print
        io::stdout().flush().expect("Failed to flush"); //dont add new line

        let mut guess = String::new(); //make a new mutable string

        io::stdin()                         //get user input
            .read_line(&mut guess)          //store in guess variable
            .expect("Failed to read line"); //send error message

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); //convert string to int

        println!("You guessed: {guess}"); //prints user guess

        match guess.cmp(&secret_number) { //compare the two numbers
            Ordering::Less => println!("Too small!"), //if guess is too big print that
            Ordering::Greater => println!("Too big!"), //if guess is too small print that
            Ordering::Equal => println!("You win!"), //if same as guess print that
        } //end match
    } //end loop
} //end main
