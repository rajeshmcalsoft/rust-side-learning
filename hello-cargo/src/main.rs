use std::{io, num};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("enter string bitch");
    let secret_number: i8 = rand::thread_rng().gen_range(1..10);
    
    loop {
        let mut guess: String = String::new();
        let _ = io::stdin().read_line(&mut guess);
        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("correct");
                break;
            },
            Ordering::Less => println!("guess higher"),
            Ordering::Greater => println!("guess lower"),
        }
    }
}