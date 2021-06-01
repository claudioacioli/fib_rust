use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    fib(30);
    let end = SystemTime::now();
    let diff = end.duration_since(start).expect("...");
    println!("time: {:?}", diff);
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n -2)
    }
}
