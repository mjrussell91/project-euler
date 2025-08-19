// Author: Matthew Russell
// Project Euler - Problem 5
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all fo the numbers from 1 to 20?


fn main() {
    let start: i32 = 2520; // 2520 in real problem
    let divisible_limit: i32 = 20;
    let mut smallest_number: i32 = start;

    let mut i: i32 = start;
    while smallest_number == start {
        let mut evenly_divisible = true;
        for j in 1..divisible_limit {
            if i % j > 0 {
                evenly_divisible = false;
            }
        }
        if evenly_divisible {
            smallest_number = i;
            println!("{} is evenly divisible by 1 to {}", i, divisible_limit)
        }
        i += 1;
    }
}