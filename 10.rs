// Author: Matthew Russell
// Project Euler - Problem 10 - Summation of Primes

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

fn main() {
    let mut sum: i64 = 0;
    // let limit: i64 = 10_i64;
    let limit: i64 = 2000000_i64;

    for i in 2..limit {
        let mut is_prime: bool = true;
        for j in 2..i.isqrt() + 1 {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{} is a prime number", i);
            sum += i;
        }
    }

    println!("The sum of all primes below {} is: {}", limit, sum);
}
