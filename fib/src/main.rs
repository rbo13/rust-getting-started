fn main() {
    println!("Fib: {}", fib(5));
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
