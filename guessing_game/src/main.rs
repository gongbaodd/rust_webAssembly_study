use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("guess a number");

    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input a number:");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct, you win!");
                break;
            }
        }
    }

}
