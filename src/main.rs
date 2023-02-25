use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_tries = 7;

    // println!("The secret number is: {secret_number}");

    let mut lose = true;

    for n in 1..max_tries {
        let tries = max_tries-n;
        println!("Please input your guess. You have more {tries} tries:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                lose = false;
                break;
            }
        }
    }
    if lose {
        println!("You lose! The number was: {secret_number}");
    }
}
