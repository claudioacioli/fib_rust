use std::time::SystemTime;
mod fibs;
mod utils;

fn main() { 
    let n = utils::get_n();
    
    let start = SystemTime::now(); 
    fibs::fib_loop(n);
    let end = SystemTime::now();
    
    println!(
        "time {:?} to calculate the fibonacci of {}", 
        end.duration_since(start).expect("..."), 
        n
    );
}
