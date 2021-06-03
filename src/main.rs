mod fibs;
mod utils;

fn main() { 
    let n = utils::get_n();

    println!("Fibonacci loop:");
    utils::compare(fibs::l_fib, n);

    println!("\nFibonacci recursive:");
    utils::compare(fibs::r_fib, n);
}