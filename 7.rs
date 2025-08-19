// Author: Matthew Russell
// Project Euler - Problem 7 - 10,000st Prime

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

fn main() {
    let limit: usize = 10000;
    let mut prime_numbers: Vec<i32> = vec![];
    let mut i: i32 = 2;
    while prime_numbers.len() <= limit {
        let mut is_prime: bool = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            prime_numbers.push(i);
        }
        i += 1;
    }
    println!("10,000st prime number: {}", prime_numbers[prime_numbers.len()-1]);
}