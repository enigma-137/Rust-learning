fn main() {
    let mut i = 0;
   loop{
    println!("loop {}", i);
    i += 1;
    if i > 4 {
        break;
    }
   
   };



   let mut x = 0;
   while x <= 4 {
    println!("While loop {}", x);
    x += 1;
   }

   for c in 0..6 {
    println!("for loop {}", c);
   }

   let arr = [1, 2, 4, 5, 6];

   let n: usize = arr.len();
   for b in 0..n {
    println!("array {}", arr[b]);
   }
}
