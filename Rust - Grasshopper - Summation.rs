// Rust - Grasshopper - Summation
// https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/rust

// fn summation(n: i32) -> i32 {
//    let mut sum: i32 = 0;
//    for i in 0..n+1 {
//        sum = sum + i;
//    }
//    sum
// }

// fn summation(n: i32) -> i32 {
//    (0..n+1).sum()
// }

fn summation(n: i32) -> i32 {
    n * (n + 1) / 2
}
