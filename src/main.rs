mod fibs;
mod utils;

fn main() {
    println!("Please input a number:");
    let n:u128 = utils::get_number();

    utils::show_results(
        "Fibonacci loop:",
        &n,
        &utils::evaluate(fibs::loop_fib, n)
    );

    utils::show_results(
        "\nFibonacci recursive:",
        &n,
        &utils::evaluate(fibs::recursive_fib, n)
    );

}