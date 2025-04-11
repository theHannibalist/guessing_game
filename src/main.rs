use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // let's see ...
    println!("The secret number is {secret_number}");
    loop {
        println!("Guess the number !");
        println!("please enter your guess :");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("unknown error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            println!("Numbers Only please !");
            continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Correct !");
                break;
            },
            Ordering::Greater => println! ("Too big !"),
            Ordering::Less => println!("too small!"),
        };

    }
}