use std::io;
use std::str::FromStr;

pub fn get_n() -> u128 {
    let mut number:String = String::new();
    println!("Please input a number:");
    io::stdin().read_line(&mut number).expect("Nothing to read...");
    u128::from_str(&number.trim()).unwrap_or(0)
//    return number.trim().parse::<u128>().unwrap_or(0);
}