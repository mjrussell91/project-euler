// Author: Matthew Russell
// Project Euler - Problem 4
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91x99.
// Find the largest palindrome made from the product of two 3-digit numbers.

#![allow(non_snake_case)]

fn checkPalindrome(number: i32) -> bool {
    let letters: Vec<char> = number.to_string().chars().collect();
    let reverse: Vec<char> = letters.clone().into_iter().rev().collect();
    return letters == reverse;
}

fn main() {
    let min_limit: i32 = 100;
    let max_limit: i32 = 999;
    let mut largestPalindrome: i32 = 1;

    let mut i: i32 = max_limit;
    while i > min_limit {
        let mut j = max_limit;
        while j > min_limit {
            let product: i32 = i * j;
            if checkPalindrome(product) && product > largestPalindrome {
                println!("{} is a palindrome", product);
                largestPalindrome = product;
            }
            j -= 1;
        }
        i -= 1;
    }
}