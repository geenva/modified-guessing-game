use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;
use std::process;
#[macro_use]
extern crate colour;

fn main() {
    println!("Welcome to the Guessing Game - guess a number between 1-10!");

    // collect cli arguments
    let args: Vec<String> = env::args().skip(1).collect();

    // skip dev-mode if vector is empty
    if args.is_empty() {
        guessing_game(false);
    } else {
        // enable dev mode if args[0] = dev
        if args[0] == "dev" {
            guessing_game(true);
        } else {
            guessing_game(false);
        }
    }
}

fn guessing_game(dev: bool) {
    // generate number from rand crate
    let num = rand::thread_rng().gen_range(1..10);

    // check if developer mode is on from function arguments
    if dev {
        yellow_ln!("Developer mode on. Proceeding...");
        yellow_ln!("The number is {}.", &num);
    }

    // initialise times variable
    let mut times = 3;

    // loop starts
    while times != 0 {
        // check if times are over

        let mut guess = String::new();
        blue_ln!("Enter your guess... ");

        // read line and apply to guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // convert guess to u32 and check if it is valid number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                red_ln!("Please enter a number.");
                continue;
            }
        };

        // remove one try
        times -= 1;

        // match guess with number
        match guess.cmp(&num) {
            Ordering::Less => println!("Too small! Your guess is {}.", &guess),
            Ordering::Equal => {
                green_ln!("Equal! You won!");
                process::exit(0)
            }
            Ordering::Greater => println!("Too big! Your guess is {}.", &guess),
        }
    }

    red_ln!(
        "😢 You ran out of tries. The number was {}. Try again!",
        num
    );
}
