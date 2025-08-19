// Author: Matthew Russell
// Project Euler - Problem 9 - Special Pythagorean Triplet

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

fn main() {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for i in 1..1000 {
        for j in i..1000 {
            for k in j..1000 {
                if i + j + k == 1000 && (i * i) + (j * j) == k * k {
                    a = i;
                    b = j;
                    c = k;
                }
            }
        }
    }

    println!("The Pythagorean triplet is: {}, {}, {}", a, b, c);
    println!("The product abc is: {}", a * b * c);
}
