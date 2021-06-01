//pub fn fib(n: u64) -> u64 {
//    if n < 2 {
//        n
//    } else {
//        fib(n - 1) + fib(n - 2)
//    }
//}

pub fn fib_loop(n: u64) -> u64 {
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
