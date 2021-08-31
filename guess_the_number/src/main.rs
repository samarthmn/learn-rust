use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..10);

    println!("Secret:{}", random_number);

    let mut i = 0;
    let max_chance = 3;
    let max_number = 10;
    let min_number = 0;

    loop {
        if i >= 3 {
            break;
        }
        println!(
            "Guess the number between {} to {} ( You have {} left ):",
            min_number,
            max_number,
            max_chance - i
        );

        let mut guessed_number: String = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read line");

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                i = i - 1;
                println!("Please enter a number");
                continue;
            }
        };

        match guessed_number.cmp(&max_number) {
            Ordering::Less => match guessed_number.cmp(&min_number) {
                Ordering::Less => {
                    println!(
                        "Please guess the number between {} to {}",
                        min_number, max_number
                    )
                }
                Ordering::Greater => {
                    match guessed_number.cmp(&random_number) {
                        Ordering::Less => println!("{}", "You guessed Smaller".yellow()),
                        Ordering::Greater => println!("{}", "You guessed Larger".yellow()),
                        Ordering::Equal => {
                            println!("{}", "You Win!!".green());
                            break;
                        }
                    }
                    i = i + 1;
                }
                Ordering::Equal => {
                    println!(
                        "Please guess the number between {} to {}",
                        min_number, max_number
                    )
                }
            },
            Ordering::Greater => {
                println!(
                    "Please guess the number between {} to {}",
                    min_number, max_number
                )
            }
            Ordering::Equal => {
                println!(
                    "Please guess the number between {} to {}",
                    min_number, max_number
                )
            }
        }
    }
}
