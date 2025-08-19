// Author: Matthew Russell
// Project Euler - Problem 3
// The prime factors of 13195 are 5, 7, 13, and 29. What is the largest prime factor of the number 600851475143

fn main() {
    let limit: i64 = 600851475143;
    let mut largest_factor: i64 = 1;

    for i in 1..limit {
        if limit % i == 0 { // if i is a factor
            for j in 1..i.isqrt() { // trial division
                if i % j == 0 && j > largest_factor { // if j is a factor and larger than the previous factor
                    largest_factor = j;
                    println!("{factor} is a prime factor of {limit}", factor=j, limit=limit);
                }
            }
        }
    }
}
