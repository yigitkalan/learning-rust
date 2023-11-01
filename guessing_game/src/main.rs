use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(1..101);
    println!("I have a secret number between 1-100 !!!");

    loop {
        print!("Enter your guess : ");
        io::stdout().flush().expect("Failed to flush stdout"); // Explicitly flush stdout
                                                               //stdout is line buffered so it automatically
                                                               //flushes when another line needed since this is print,
                                                               //not println we need to do i manually

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // this is shadowing, the line below shadows the previous decleration of guess
        // shadowing generally used to change the type of the variable to another
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a valid number\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
