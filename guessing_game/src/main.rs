use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guessing game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input a number");

        let mut guess = String::new();

        io::stdin().
        read_line(& mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        
    };

    match guess.cmp(& secret_number){
        Ordering::Greater => print!("Too high"),
        Ordering::Less => print!("Too low"),
        Ordering::Equal => {
            print!("You got it!");
            break;
        }
    }
    

    }
}