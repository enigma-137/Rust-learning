fn main() {
//    let v:Vec<u32> = Vec::new();

   let mut v2 = vec![1,2,3,4,5, 6];

//    match v2.get(30) {
//     Some(third) => println!("The third value is {}", third),
//     None => println!("There is no third value")
//    }
for i in &mut v2{
    *i += 50
}

for i in &v2{
    println!("{}", i);
}
}
