// use std::fs::File;
// use std::io::ErrorKind;


fn main() {

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Error creating file {:?}", other_error);
    //         }
    //     }
    // };
    // b();
// match x {a => b, c =>d}, if a return b, if c return d.

    let x: Option<i32> = Some(9);

    match x {
        Some(val) => println!("Result is {val}"),
        None => println!("None!")
    }

    if let Some(val) = x {
        println!("Result is {val}");
    }
    // let res: Result<i32, String> = Ok(200);
    // let res: Result<i32, String> = Err("Something broke".to_string());

    // match res {
    //     Ok(r) => println!("Request successful with {}", r),
    //     Err(e) => println!("Error: {}", e),
    // }

    let config: Option<u32> =Some(42);
    let mut value = 0;

    if let Some(val) = config{
        value = val * 2
    }
    print!("Processed value: {}", value)
}
// fn b(){
//     c(22)
// }

// fn c(num: i32){
//     if num == 22 {
//         panic!("Don't pass 22 please")
//     } 
// }
