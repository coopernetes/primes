// The below is generated from ChatGPT.
// Prompt: "What is the most efficient algorithm to calculate whether a number is prime?"

// There are several algorithms to determine whether a number is prime, with varying degrees 
// of efficiency depending on the size of the number and the specific implementation.
// 
// One of the most efficient algorithms for determining whether a number is prime is the 
// Miller-Rabin primality test. The Miller-Rabin primality test is a probabilistic algorithm
// that can quickly determine whether a number is definitely composite or likely to be prime.
// The algorithm is based on the observation that if n is a prime number, then for any integer a,
// a raised to the power of n-1 modulo n is congruent to 1.

// The Miller-Rabin primality test works by choosing a random base a and then checking whether a
// raised to the power of n-1 modulo n is congruent to 1 or not. If it is not congruent to 1, then
// n is definitely composite. If it is congruent to 1, the algorithm proceeds to check other
// conditions.

// The Miller-Rabin primality test can be run in polynomial time and has a very small probability
// of returning a false positive. However, the probability of a false positive can be reduced by
// performing multiple iterations of the algorithm with different random bases.

// Prompt: "Write the algorithm in Rust."

use rand::Rng;


// This implementation uses the rand crate to generate random numbers. The miller_rabin function
// takes two arguments: the number to be tested (n) and the number of iterations to perform (k).
// The function returns a boolean value indicating whether n is prime or not.

// The implementation first checks for some base cases where the input is either 2 or 3, and returns
// true in those cases. If the input is less than 2 or even, it returns false.

// Next, the function computes d and s such that n-1 = d * 2^s, where d is an odd integer. It then
// generates k random bases a and performs the Miller-Rabin test on each one.

// For each base a, the function checks if a^d mod n is equal to 1 or n-1. If it is, it proceeds to
// the next iteration with a new random base. Otherwise, it computes a^(2^r * d) mod n for r from 1
// to s-1, and checks if any of the results is equal to n-1. If none of them are, the function 
// returns false, indicating that n is composite. Otherwise, it continues with the next iteration 
// using a new random base.

// If all iterations pass without returning false, the function returns true, indicating that n is probably prime.
fn miller_rabin(n: u64, k: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n < 2 || n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;

    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(2..n - 2);
        let mut x = a.pow(d as u32) % n;

        if x == 1 || x == n - 1 {
            continue;
        }

        for _ in 1..s {
            x = (x * x) % n;

            if x == n - 1 {
                break;
            }
        }

        if x != n - 1 {
            return false;
        }
    }

    true
}
