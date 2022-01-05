// Rust - Sum of positive
// https://www.codewars.com/kata/5715eaedb436cf5606000381/train/rust

fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&&i| i > 0).sum()
}
