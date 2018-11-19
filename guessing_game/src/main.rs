extern crate rand; //external dependency

use std::io; //io library comes from the standard library. By default Rust only brings in a few types via prelude
use rand::Rng; //Rng is a trait. Trait must be in scope to use it's methods
use std::cmp::Ordering; //Ordering is an enum (ADT)

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng(); //thread_rng returns ThreadRng which has implementations for Rng. mut???
    let secret_number = rng.gen_range(1,101); //in order to use gen_range we need to import Rng trait

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new(); //:: means new is an associated function of String type
        let stdin = io::stdin(); //stdin function returns an instance of std::io::Stdin file:///Users/laiboonhui/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/std/io/fn.stdin.html
        stdin.read_line(&mut guess).expect("Fail to read line"); // & means reference. It lets multiple parts of the code access one piece of data without needing to copy into memory multiple times. Like variables, references are immutable by default
        
        let guess: u32 = match guess.trim().parse() { //Rust defaults to i32 if you do not specify numeric type. We need to trim if not it will input the newline character when you press enter after keying in your guess
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) { //file:///Users/laiboonhui/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/std/primitive.str.html#method.cmp
            Ordering::Less => println!("Too small"), //notice the comma
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        };
    }  
}
