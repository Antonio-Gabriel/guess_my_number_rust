extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Insert a palpite: ");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("An error occurred to read the input");

        // let palpite: u32 = palpite.trim().parse().expect("Please, input a number");
  
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You say: {}", palpite);

        match palpite.cmp(&secret_number) {
            Ordering::Less => println!("Very small!"),
            Ordering::Greater => println!("Very large"),
            Ordering::Equal => {
                println!("Are equals, you win");
                break;
            }
        }
    }
}
