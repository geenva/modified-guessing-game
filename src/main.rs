use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game.");
    let mut args: Vec<String> = Vec::new();
    for i in env::args().skip(1) {
        args.push(i)
    }

    if args.len() == 0 {
        guessing_game(false);
    } else {
        if args[0] == "dev" {
            guessing_game(true);
        } else {
            guessing_game(false);
        }
    }
}

fn guessing_game(dev: bool) {
    let num = rand::thread_rng().gen_range(1..10);

    if dev == true {
        println!("WARN: DEVELOPER MODE ON.");
        println!("The number is {}.", &num)
    }

    loop {
        let mut guess = String::new();
        println!("Enter your guess... ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please type a number.");

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small! Your guess is {}.", &guess),
            Ordering::Equal => {
                println!("Equal! You win! Your guess is {}.", &guess);
                break;
            }
            Ordering::Greater => println!("Too big! Your guess is {}.", &guess),
        }
    }
}
