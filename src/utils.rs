use std::io;

pub fn get_n() -> u64 {
    let mut string_number = String::new();
    println!("Please input a number:");
    io::stdin().read_line(&mut string_number).expect("Nothing to read..");
    let n = string_number.trim().parse::<u64>().unwrap();
    n
}
