use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;


fn main() 
{
    
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secret number: {}",secret_number);
    loop {

        let  mut guess  = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Filed to read line");

    println!("You guessed: {}",guess);
    println!("Random number: {}",secret_number);

    let guess:i32= match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


    match guess.cmp(&secret_number)
    {   Ordering::Less => println!("{}","To small".red()),
        Ordering::Greater => println!("{}","To big".red()),
        Ordering::Equal => {
            println!("{}","You win".green());
            break;
        },
    }
    }
}