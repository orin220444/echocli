use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("type something");
        let mut usertypings = String::new();
        io::stdin()
            .read_line(&mut usertypings)
            .expect("Failed to read line");
        let usertypings = usertypings.trim_end();
        if usertypings == "quit" || usertypings == "exit" {
            println!("exiting...");
            break;
        } else if usertypings == "game" {
            game();
            break;
        } else {
            println!("you typed: {}", &usertypings);
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
