#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use rand::Rng;
use std::io::stdin;

fn if_statement() {
    println!("If statements:");
    let temp = 35;

    if temp > 30 {
        println!("Really hot outside!");
    } else if temp < 10 {
        println!("Really cold!");
    } else {
        println!("Temperature is OK!");
    }

    // If else statements can be used when declaring a variable
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("Today is {}", day);

    // If else statements can also be used in another statement, and they can be nested
    println!(
        "It is {}",
        if temp > 20 {
            if temp > 30 {
                "very hot"
            } else {
                "hot"
            }
        } else if temp < 10 {
            "cold"
        } else {
            "ok"
        }
    )
}

fn while_statement() {
    println!("While statements:");
    let mut x = 1;

    while x < 1000 {
        // The variable x multiplies by 2 every iteration
        x *= 2;
        // The continue statement will skip the following codes and goes to the next iteration
        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    // Basically while true
    loop {
        y *= 2;
        println!("y = {}", y);
        // The break statement will stop the loop
        // 1 << 10 means 2 ^ 10 because it is 1 in binary shifted to the left by 10 digits
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_statement() {
    println!("For loop statements:");
    // for loop from 1 to 10
    // The 11 means the upper bound and will not be executed
    for x in 1..11 {
        // Again, continue and break statements can work in for loop
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    // If you want to traverse a range (i.e., 30 to 40) with the position that would start from zero,
    // you would use enumerate to create pairs of values where
    // the first value is the position and the second value is from 30 to 40
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    println!("Match statements:");
    // A match statement works like a switch statement from other programming language
    let country_code = 1001;
    // The rust compiler will want you to ensure all possible cases are being covered
    // Commenting out the underscore will raise an error
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        86 => "China",
        1 => "USA",
        1..=1000 => "unknown", // The equal makes it an inclusive range
        _ => "invalid",        // The underscore served as catch all cases
    };

    println!("The country with code {} is {}", country_code, country);
}

fn combination_lock() {
    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    // The code is the correct password for the lock, the state is the lock, and the entry is user's input
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    // Loop for multiple user's attempts to unlock the lock
    println!("Please enter the password one digit at a time:");
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                // The read_line() return something called io::Result<usize>
                // A Result is a result of T and Error. This result is an enumeration
                // Either you will get an "Ok" that contains the valid value, or error

                // Here since we try to match a read_line() output, we need to cover both
                // Ok and Err results
                match stdin().read_line(&mut input) {
                    // This will add user's input to the end of the entry, and each loop will add one digit
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue,
                }
                // Unlock the state if the entry is the password
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                // This will check if the current entry correctly contains part of the password or not
                // For example, if the entry is 12 and user typed 5, it doesn't match the first three digit
                // of our password 1234, hence the state will become failed.
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            // If attempt is failed, then reset the entry and the lock
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            // If the lock is unlocked, then end the program
            State::Unlocked => {
                println!("UNLOCKED");
                entry.clear();
                return;
            }
        }
    }
}

fn main() {
    if_statement();
    while_statement();
    for_statement();
    match_statement();
    combination_lock();
}
