use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    loop {

    println!("Enter a number to guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(& mut guess)
    .expect("Failed to read line");

    // let guess: u32 = guess.trim().parse()
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }; 
    
    //.expect("Please enter a number");
println!("Your guess is: {guess}");

 match guess.cmp(&secret_number)
{
    Ordering::Less => println!("Too low"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {
        println!("You Got it");
        break;
        }
}
}}