use std::io; //import io from standard library
use std::io::Write; //import print stuff

fn main() { //define the function main
    println!("Guess the number!"); //print

    print!("Please input your guess. "); //another print
    io::stdout().flush().expect("Failed to flush"); //dont add new line

    let mut guess = String::new(); //make a new mutable string

    io::stdin()                         //get user input
        .read_line(&mut guess)          //store in guess variable
        .expect("Failed to read line"); //send error message

    println!("You guessed: {guess}"); //prints user guess
}
