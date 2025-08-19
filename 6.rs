// Author: Matthew Russell
// Project Euler - Problem 6 - Sum Square Difference

// The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
// The square sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640
// Find the difference between he sum of the squares of the first one hundred natural numbers and the square of the sum

fn main() {
    let limit: i32 = 100;

    let mut sum: i32 = 0;
    let mut square_sum: i32 = 0;
    for i in 1..limit+1 {
        sum += i;
        square_sum += i * i;
    }
    sum = sum * sum;
    println!("sum squared {}, square sum {}", sum, square_sum);
    println!("difference between sum and square of the sum = {}", sum - square_sum);
}