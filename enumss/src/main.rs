// enum IpAddressKind {
// v4,
// v6,
// }

// struct IpAddress {
//     kind: IpAddressKind,
//     address: String
// } 
// fn main() {
//     // specify variances
//     let four = IpAddressKind::v4;
//     let five = IpAddressKind::v6;
// // create the variable for the local host ip address
//     let localhost = IpAddress {
//         kind: IpAddressKind::v4,
//         address: String::from("127.0.0.1"),
//     };
// }

// fn route(ip_kind: IpAddressKind){

// }


#[derive(Debug)]
// enum LoadingState {
//     Loading,
//     Success,
//     Failure,
// }
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 12, y: 18 };
    let msg3 = Message::Write(String::from("Enigma"));
    let msg4 = Message::ChangeColor(12, 18, 24);


    msg3.print();
}

impl Message {
    fn print(&self){
        println!("Message {:?}", self)
    }
}
