fn main() {
    let string1 = String::from("A, B, C, D");
    let string2 = String::from("A, B, C, ");

    let result = longest(string1.as_str(), string2.as_str());
    println!("{}", result)
}
// generic life times
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
