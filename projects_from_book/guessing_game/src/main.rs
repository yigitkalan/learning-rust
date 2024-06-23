use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(1..101);
    println!("I have a secret number between 1-100 !!!");

    loop {
        print!("Enter your guess : ");
        io::stdout().flush().expect("Failed to flush stdout"); 
        // Explicitly flush stdout stdout is line buffered so it 
        // automatically flushes when another line needed since 
        // this is print, not println we need to do i manually

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");


        // this is shadowing, the line below shadows the previous decleration of guess
        // shadowing generally used to change the type of the variable to another
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter a valid number\n");
                continue;
            }
        };

        let guess = Guess::new(guess);


        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}


pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
        if !(0..=100).contains(&value){
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { 
            value
        }
    }
    pub fn value(&self) -> i32{
        self.value
    }
}
