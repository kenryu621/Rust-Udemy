use rand::Rng;
use std::io;

fn generate_number() -> i64 {
    rand::rng().random_range(1..101)
}

fn check_guess(secret_num: i64, guess: i64) -> bool {
    if guess < 1 || guess > 100 {
        println!("Your guess is out of range!")
    } else if guess > secret_num {
        println!("Guess lower.")
    } else if guess < secret_num {
        println!("Guess higher.")
    } else if guess == secret_num {
        println!("You got it right!")
    }
    secret_num == guess
}

fn get_user_input() -> Result<i64, std::num::ParseIntError> {
    println!("Guess the number: ");
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");
    input_line.trim().parse::<i64>()
}

fn main() {
    let secret_num = generate_number();
    loop {
        match get_user_input() {
            Ok(guess) => {
                if check_guess(secret_num, guess) {
                    break;
                };
            }
            Err(_) => {
                println!("Please enter a valid integer!");
                continue;
            }
        }
    }
}
