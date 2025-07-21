use std::thread;
use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let start = Instant::now();
    let _ = fibonacci(45);
    let duration = start.elapsed();
    println!("fibonacci(45) in {:?}", duration);

    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..8 {
        let handle = thread::spawn(|| fibonacci(45));
        handles.push(handle);
    }

    for handle in handles {
        println!("Handle: {:?}", handle);
        let result = handle.join();
        match result {
            Ok(value) => println!("Thread finished with value: {}", value),
            Err(e) => println!("Thread failed with error: {:?}", e),
        }
    }
    let duration = start.elapsed();
    println!("fibonacci(45) in {:?}", duration);
}
