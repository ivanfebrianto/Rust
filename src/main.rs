extern  crate rand;
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess The Number! HOHOHO");
    let secret_number = rand::thread_rng().gen_range(1..10);
    println!("The Secret Number is : {}",secret_number);  //You can hide this message
    loop {
        
        println!("Please Input a Number: ");
        let mut guess = String::new(); 
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line!!");
        let guess : i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You Guessed: {}",guess);
        match guess.cmp(&secret_number) {
            Ordering ::Less => println!("Too Small!"),
            Ordering ::Greater => println!("Too Big!"),
            Ordering ::Equal => {
                println!("You Win!");
                break;
            }   
        }
}
}
