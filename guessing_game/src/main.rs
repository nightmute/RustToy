extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("guess the Number!");
    // random number generator
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The guess num : {} ", secret_number);

    loop{
        println!("Please enter the number");
        let mut number = String::new();
        io::stdin().read_line(&mut number)
            .expect("failed to read");
        
        let guess: u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("The number : {}", number);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is less than the secret number"),
            Ordering::Greater => println!("The number is greater than the secret number"),
            Ordering::Equal => {
                println!("The number is equal to the secret number");
                break;
            }
        }    
    }
    




}
