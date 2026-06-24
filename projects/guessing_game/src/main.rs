use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // hides the secret number println!("The secret number is: {secret_number}");

    loop {    
        println!("Please input your guess.");

        let mut guess = String::new(); // type with function new

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // trim removes whitespace at start and end of a string, and also removes newline \n and in Windows - the carriage return and newline  \r\n
        let guess: u32 = match guess.trim().parse()  {  // parse converts a type to another type   
            Ok(num) => num,   
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {   // expression is made up of arms. An arm consists of pattern to match against.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }     
        }
    }
}
