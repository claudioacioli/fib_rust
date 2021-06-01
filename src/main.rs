use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    fib(100);
    let end = SystemTime::now();
    let diff = end.duration_since(start).expect("...");
    println!("time: {:?}", diff);
}

fn fib(n: u128) -> u128 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n -2)
    }
}
