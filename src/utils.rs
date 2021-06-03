use std::io;
use std::str::FromStr;
use std::time::{SystemTime, Duration};

pub fn get_number() -> u128 {
    let mut number:String = String::new();
    io::stdin().read_line(&mut number).expect("Nothing to read...");
    u128::from_str(&number.trim()).unwrap_or(0)
//    return number.trim().parse::<u128>().unwrap_or(0);
}

pub fn evaluate(f: fn(u128) -> u128, n:u128) -> Duration {
    let start = SystemTime::now();
    f(n);
    let end = SystemTime::now();
    end.duration_since(start).expect("...")
}

pub fn show_results (title:&str, number:&u128, duration:&Duration) {
    println!(
        "{}\ntime {:?} to calculate the fibonacci of {}",
        &title,
        &duration,
        &number
    );
}