mod fibs;
mod utils;

fn main() {
    println!("Please input a number:");
    let n:u128 = utils::get_number();

    println!("Fibonacci loop:");
    utils::evaluate(fibs::loop_fib, n);

    println!("\nFibonacci recursive:");
    utils::evaluate(fibs::recursive_fib, n);
}