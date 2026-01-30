fn fibonacci(n: u32) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut prev: u128 = 0;
    let mut curr: u128 = 1;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

fn main() {
    let n = 10;
    println!("The {}th Fibonacci number is {}", n, fibonacci(n));
}
