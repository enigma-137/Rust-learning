fn main() {
//    let v:Vec<u32> = Vec::new();

   let v2 = vec![1,2,3,4,5];

//    match v2.get(30) {
//     Some(third) => println!("The third value is {}", third),
//     None => println!("There is no third value")
//    }

for i:i32 in &v2{
    println!("{}", i);
}
}
