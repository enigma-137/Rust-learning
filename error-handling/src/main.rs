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


    let x: Option<i32> = Some(9);

    match x {
        Some(val) => println!("{val}"),
        None => println!("None!")
    }
    let res: Result<i32, String> = Ok(200);
    // let res: Result<i32, String> = Err("Something broke".to_string());

    match res {
        Ok(r) => println!("Request successful with {}", r),
        Err(e) => println!("Error: {}", e),
    }
}
// fn b(){
//     c(22)
// }

// fn c(num: i32){
//     if num == 22 {
//         panic!("Don't pass 22 please")
//     } 
// }
