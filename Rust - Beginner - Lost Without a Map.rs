// Rust - Beginner - Lost Without a Map
// https://www.codewars.com/kata/57f781872e3d8ca2a000007e/train/rust

fn maps(values: &Vec<i32>) -> Vec<i32> {
    values.iter().map(|x| x*2).collect()
}
