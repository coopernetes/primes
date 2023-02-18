use std::io;
use std::io::Write;
use std::time::Instant;
use std::env;

fn main() {
    let start = Instant::now();
    let mut max: i32 = 1000;
    let str_vars = env::args().collect::<Vec<String>>();
    if str_vars.len() > 1 {
        let arg = &str_vars[1];
        if arg.len() != 0 {
            max = arg.parse().unwrap();
        }
    }
    let mut i: i32 = 1;
    let mut primes = vec![];
    while i <= max {
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
            primes.push(i);
            out.push_str("yes\t\r");
        } else {
            out.push_str("no \t\r");
        }
        io::stdout().write(out.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        i += 1;
    }
    let end = start.elapsed();
    println!("\nCount: {}", primes.len());
    println!("Time (ns): {}", end.as_nanos());
    println!("Time (µs): {:.3}", end.as_nanos() as f64 / 1_000.0);
    println!("Time (ms): {:.3}", end.as_nanos() as f64 / 1_000_000.0);
    println!("Time (s): {:.4}", end.as_nanos() as f64 / 1_000_000_000.0);
}
