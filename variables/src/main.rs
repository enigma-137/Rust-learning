fn main() {
//   let sum = my_function(1, 2);
//   println!("The sum is: {}", sum)
// }


// fn my_function(x: i32, y: i32) -> i32 {
//     println!("The value of X is: {}", x);
//     println!("The value of Y is: {}", y);
//      x + y
    
// let mut counter = 0;
//     let result = loop{
//         counter += 1;

//         if counter == 10{
//             break counter;
//         }
        


//     };
//     println!{"The bomb will explode in: {}", result}

//     let mut number = 10;
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//      }


    

//     println!("Blowwwwwwwwwwww BOOOOOM!!")
let m1 = String::from("Hello");
let m2 = String::from("World");

greet(&m1, &m2);
let s = format!("{} {}", m1, m2);
println!("{}", s);


}

fn greet(g1: &String, g2: &String){
    println!("{} {}", g1, g2);}
