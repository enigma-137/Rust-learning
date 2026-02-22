use std::fs::File;
use std::io::ErrorKind;


fn main() {

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file {:?}", e),
            },
            other_error => {
                panic!("Error creating file {:?}", other_error);
            }
        }
    };
    // b();
}
// fn b(){
//     c(22)
// }

// fn c(num: i32){
//     if num == 22 {
//         panic!("Don't pass 22 please")
//     } 
// }
