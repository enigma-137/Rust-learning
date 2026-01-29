use std::io;


fn f_to_c(f: f64) -> f64 {
    (f - 32.0) - 5.0/9.0

}

fn c_to_f(c: f64)-> f64 {
    (c * 9.0/5.0) / 9.0
}

fn main() {

  println!("Choose conversion");
  println!("1: For fahrenheit to Celcius");
  println!("2: For Celcius to fehrenheit");

  let mut choice = String::new();

  io::stdin().read_line(& mut choice).unwrap();

  let choice = choice.trim();


  println!("Enter temperature value");

  let mut input = String::new();
  io::stdin().read_line(& mut input).unwrap();

  let value: f64 = input.trim().parse().unwrap();

  match choice {
    "1" => {
        let result = f_to_c(value);
        println!("{value}°F = {result:.2}°C");
    }
    "2" => {
        let result = c_to_f(value);
        println!("{value}°C = {result:.2}°F");
    }
    _=> println!("Invalid choice")
  }





    
    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an index")

    // let mut index = String::new();

    // io::stdin()
    // .read_line(& mut index)
    // .expect("This is not what we expected");

    // let index: usize = index.trim().parse().expect("Number too not so okay");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
    // let mut counter = 0;

    // let result = loop{
    //     counter += 1;
    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };

    // println!("The result is: {result}")

}