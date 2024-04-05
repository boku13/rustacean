use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    let num = rand::thread_rng().gen_range(1, 101);

    loop{
    println!("Please input your guess.");
    
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess).
    expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed : {}", guess);

    match guess.cmp(&num){
        Ordering::Less => println!("{}", "too small!".red()),
        Ordering::Equal => {
            println!("{}", "that's right!".green());
            break;
        },
        Ordering::Greater => println!("{}", "too big".red()),
    }
    }
}
