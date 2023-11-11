
use rand::Rng;
use std::cmp::Ordering;
use std::io;




fn main() {
    println!("Welcome to: GUESS THE NUMBER GAME");

    // Generating the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your Secret Number is {}", secret_number);
    

    let mut tries = 0;

    loop {
        println!("Please Input Your Guess Number ...");

        tries += 1;

        let mut guess = String::new();

        io::stdin()
                .read_line(&mut guess)
                .expect("Unable to parse input!");
        
        if guess.trim().to_lowercase() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number between 1 and 100");
                continue;
            }
        };
            // let guess: u32 = guess
            //     .trim()
            //     .parse()
            //     .expect("Please input a number between 1 and 100");
            println!("Your guess is {guess}");

        
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big"),
                Ordering::Equal => {
                println!("You win! Took you {tries} tries to guess the secret number!");
                break;
                }
            }

    }

}
