// Rust - Multiples of 3 or 5
// https://www.codewars.com/kata/514b92a657cdc65150000006/solutions/rust

fn solution(num: i32) -> i32 {
    let mut i = 0;
    let mut sum = 0;

    while i < num {       
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
        i = i + 1;
        }
    sum
}
