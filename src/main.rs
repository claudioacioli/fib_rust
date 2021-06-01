use std::time::SystemTime;
use std::io;

fn main() { 
    let n = get_n();
    let start = SystemTime::now(); 
    fib(n);
    let end = SystemTime::now();
    let diff = end.duration_since(start).expect("...");
    println!("time {:?} to calculate the fibonacci of {}", diff, n);
}

fn get_n() -> u64 {
    let mut string_number = String::new();
    println!("Please input a number:");
    io::stdin().read_line(&mut string_number).expect("Nothing to read..");
    let n = string_number.trim().parse::<u64>().unwrap();
    n
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
