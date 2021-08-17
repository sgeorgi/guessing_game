use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secret_number);

    const ROUNDS: u8 = 5;
    let mut round: u8 = 0;

    loop {
        round += 1;

        if round > ROUNDS {
            println!("\nYou didn't guess the number {} in {} rounds, bye!", secret_number, ROUNDS);
            break;
        } else {
            println!("\nRound {}/{}", round, ROUNDS);
        }

        println!("Please input your guess (0 to quit).");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Bye!");
                    break;
                };
                num
            }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
