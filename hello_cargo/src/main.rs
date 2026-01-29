use std::io;

fn main() {
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an index");

    let mut index = String::new();

    io::stdin()
    .read_line(& mut index)
    .expect("This is not what we expected");

    let index: usize = index.trim().parse().expect("Number too not so okay");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}