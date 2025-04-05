use rand::Rng;
use std::{cmp::Ordering, io};

pub fn guess_number() {
    println!("####### Guess the Number #######\n");

    let secret_num = rand::thread_rng().gen_range(0..=100);
    let mut count = 0;
    loop {
        let mut guess = String::new();

        println!("\nPlease input your guessed number:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        count += 1;
        // this will crash our program if input is not a number
        // let guess: u32 = guess.trim().parse().expect("Unable to parse");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input valid number");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => {
                println!("Guess Bigger, too small")
            }
            Ordering::Equal => {
                println!("You won : {count} iteration ");
                break;
            }
            Ordering::Greater => {
                println!("Guess Smaller, too big")
            }
        };
    }
}
