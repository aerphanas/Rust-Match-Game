use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Guessing Game");

    loop {

        let mut guess = String::new();

        print!("please enter your number : ");
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("please input correct number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error : {}", e);
                break;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("Your Right");
                break;
            },
            Ordering::Greater => println!("Hint : Too Big"),
            Ordering::Less => println!("Hint : Too Less")
        }

    }

}
