pub fn recursive_fib(n: u128) -> u128 {
   if n < 2 {
       n
   } else {
       recursive_fib(n - 1) + recursive_fib(n - 2)
   }
}

pub fn loop_fib(n: u128) -> u128 {
    let mut i = 0;
    let mut x = 1;
    let mut y = 0;
    let mut aux;

    while i < n {
        aux = x + y;
        y = x;
        x = aux;
        i = i + 1;
    }
    
    x
}