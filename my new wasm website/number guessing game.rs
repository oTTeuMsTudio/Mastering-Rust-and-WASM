use rand::Rng;
use std::io;

fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn read_user_input() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().parse().expect("Please type a number!")
}

fn compare_numbers(user_input: u32, random_number: u32) -> String {
    if user_input > random_number {
        return String::from("Too high!");
    } else if user_input < random_number {
        return String::from("Too low!");
    } else {
        return String::from("You win!");
    }
}
const MAX_ATTEMPTS: u32 = 5;

fn main() {
    println!("Number guessing game");

    let random_number = generate_random_number();
    let mut attempts = 0;

    loop {
        let user_input = read_user_input();

        let text = compare_numbers(user_input, random_number);

        println!("{}", text);

        attempts += 1;

        if user_input == random_number {
            break;
        }

        if attempts == MAX_ATTEMPTS {
            println!("You lose! The number was: {}", random_number);
            break;
        }
    }
}
