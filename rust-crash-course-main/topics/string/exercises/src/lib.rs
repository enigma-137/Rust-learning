pub fn hello() -> String {
    "Hello, world!".to_string()
}

pub fn greet(name: &str) -> String {
   format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    s += "!";
    s
}
