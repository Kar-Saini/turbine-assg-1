use std::{i32, io};

use rand::Rng;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..=100);
    let mut count: i32 = 0;
    println!("Guess a number");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error");
        let parsed_guess: i32 = guess.trim().parse().expect("Failed");

        if parsed_guess < random_num {
            println!("Less, enter again, count - {} ", count)
        } else if parsed_guess > random_num {
            println!("More, enter again, count - {} ", count)
        } else if parsed_guess == random_num {
            println!("You Won, total attempt {}", count);
            break;
        }
        count += 1;
        println!("******")
    }
}
