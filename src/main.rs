extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = set_up_game();
    run_game(secret_number);
}

fn set_up_game() -> u32 {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    secret_number
}

fn run_game(secret_number: u32) -> () {
    loop {
        let guess = match read_guess() {
            Some(num) => num,
            None => {
                println!("Please enter a number 1 to 100!");
                continue;
            }
        };

        let (should_continue, string) =
            get_hint_string(secret_number, guess);

        println!("{}", string);

        if !should_continue {
            break;
        }
    }
}

fn read_guess() -> Option<u32> {
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess_num = guess.trim().parse::<u32>();

    match guess_num {
        Ok(num) => {
            if num > 0 && num <= 100 {
                Some(num)
            } else {
                None
            }
        },
        Err(_) => None
    }
}

fn get_hint_string(answer: u32, guess: u32) -> (bool, &'static str) {
    match guess.cmp(&answer) {
        Ordering::Less => (true, "Too small!"),
        Ordering::Greater => (true, "Too Large!"),
        Ordering::Equal => (false, "YOU GOT IT YA LIL WINNER YOU!!@!!!!")
    }
}
