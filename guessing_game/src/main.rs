use std::io;

fn main() {
    println!("Hiii :3, welcome to the guessing game! <3333");

    println!("Please enter a number from 1-10!!!!");
   
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    println!("You guessed : {guess}")
    
}
