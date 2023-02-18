use std::io;
use std::io::Write;
use std::time::Instant;
use rust_decimal::prelude::*;

fn main() {
    let start = Instant::now();
    let max: i32 = 1000;
    let mut i: i32 = 1;
    let mut primes = vec![];
    while i < max {
        let mut out = format!("Is {} prime?\t", i);
        let mut prime = true;
        for x in &primes {
            if *x == 1 || *x == i {
                continue
            }
            if i % x == 0 {
                prime = false;
                break
            }
        }
        if prime {
            primes.push(i)
        }
        out.push_str(format!("{}\t\r", prime).as_str());
        io::stdout().write(out.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        i += 1;
    }
    let end = start.elapsed();
    println!("\nCount: {}", primes.len());
    println!("Time (ns): {}", end.as_nanos());
    println!("Time (µs): {}", Decimal::new(end.as_nanos().try_into().unwrap(), 2));
    println!("Time (ms): {}", Decimal::new(end.as_nanos().try_into().unwrap(), 5));
    println!("Time (s): {}", Decimal::new(end.as_nanos().try_into().unwrap(), 8));
}
