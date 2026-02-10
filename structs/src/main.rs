// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
fn calc(&self) -> u32{
    self.width * self.height
}
}

fn main() {


    let rect = Rectangle {
        width: 32,
        height:40

    };

    println!("Rect: {:#?} ", rect);


    
    println!("The area of the rectangle is {} square pixels", rect.calc() );
    // let mut user1 = User{
    //     email: String::from("shola@gmail.com"),
    //     username: String::from("Sholexy"),
    //     sign_in_count: 1,
    //     active: true
    // };

    // println!("The email is {}", user1.email)
//     user1.username = String::from("Jose sholly");
//     let name = user1.username;
//     println!("The name is {}", name);

//     let user2 = build_user(String::from("toyo@gmail.com"), String::from("t4_toyo"));
//     println!("The second user is {}", user2.username);

//     let user3 = User{
//           email: String::from("Esther@gmail.com"),
//         username: String::from("Esther"),
//         ..user2
//     }
// }


// fn build_user(email: String, username: String) -> User{
//    User {
//      email, 
//     username,
//     active: true,
//     sign_in_count: 1
//    }


}

