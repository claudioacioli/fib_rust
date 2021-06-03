use std::io;
use std::str::FromStr;
use std::time::SystemTime;

pub fn get_n() -> u128 {
    let mut number:String = String::new();
    println!("Please input a number:");
    io::stdin().read_line(&mut number).expect("Nothing to read...");
    u128::from_str(&number.trim()).unwrap_or(0)
//    return number.trim().parse::<u128>().unwrap_or(0);
}

pub fn compare (f: fn(u128) -> u128, n:u128) {
    let start = SystemTime::now();
    f(n);
    let end = SystemTime::now();
    println!(
        "time {:?} to calculate the fibonacci of {}",
        end.duration_since(start).expect("..."),
        n
    );
}