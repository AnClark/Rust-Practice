fn main() {
    println!("Fibonacci array with recursive:");

    for i in (0..30) {
        let num = fibonacci(i);
        print!("{} ", num);
    }

    println!("");
}

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2); 
    }
}

