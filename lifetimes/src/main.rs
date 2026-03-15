fn main() {
    let string1 = String::from("A, B, C, D");
    let string2 = String::from("A, B, C, ");

    let result = longest(string1.as_str(), string2.as_str());
    println!("{}", result)
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
