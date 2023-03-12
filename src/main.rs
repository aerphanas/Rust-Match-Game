use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game");
    println!("please enter your number : ");
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {random_number}");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("your right");
                break;
            },
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too less")
        }
    }
}
