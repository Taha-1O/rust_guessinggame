use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess a number : ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The number you have guessed is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
            Ordering::Less => println!("Too small"),
        }
    }
}
