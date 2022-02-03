// Rust - Highest and Lowest
// https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust 

fn high_and_low(numbers: &str) -> String {
    let numbers_array: Vec<i32> = numbers.split_whitespace().map(|x| x.parse().unwrap()).collect();
    format!("{} {}", numbers_array.iter().max().unwrap(), numbers_array.iter().min().unwrap())
}
