use std::cmp::Ordering;
use std::io::{self, Write};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("\nGuess the Number! (Between 1-100) between you and me, it's {secret_number}");

    loop {
        print!("\nEnter your guess: ");
        io::stdout()
            .flush()
            .expect("How can you fail a flush? Anyways");

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("How can you fail a read? Anyways");

        let guess: u32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            }
        }
    }
}
