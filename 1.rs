// Author: Matthew Russell
// Project Euler - Problem 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

// Iterates to the limit (i) over a vector of numbers (n) that will i modulus n with a remainder of 0. These multiples will be pushed to a sum vector (sums).
// The sum vector will be summed (sum) after the loop to return the answer to the problem.

fn main() {
    let limit: i32 = 1000;
    let mut sums: Vec<i32> = vec![];
    let numbers: Vec<i32> = vec![3, 5];

    for i in 1..limit {
        for n in &numbers {
            if i % n == 0 {
                println!("{i} is a multiple of {n}", i=i, n=n);
                if !sums.contains(&i) {
                    sums.push(i);
                }
            }
        }
    }

    let sum: i32 = sums.iter().sum();
    println!("The sum of multiples is: {sum}", sum=sum);
}
