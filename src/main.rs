use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("type something");
        let mut user_typings = String::new();
        io::stdin()
            .read_line(&mut user_typings)
            .expect("Failed to read line");
        let user_typings = user_typings.trim_end();
        if user_typings == "quit" || user_typings == "exit" {
            println!("exiting...");
            break;
        } else if user_typings == "game" {
            game();
            break;
        } else {
            println!("you typed: {}", &user_typings);
        }
    }
}
fn game() {
    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(0, 51);
    let answer: String = "Okay, let`s play the game. Guess number I made".to_string();
    println!("{}", &answer);
    loop {
        let mut user_number = String::new();
        io::stdin()
            .read_line(&mut user_number)
            .expect("Failed to read line");
        let user_number: u32 = user_number.trim().parse().expect("Please type a number!");
        match user_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
