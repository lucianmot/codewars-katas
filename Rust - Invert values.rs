// Rust - Invert values
// https://www.codewars.com/kata/5899dc03bc95b1bf1b0000ad/train/rust

//  fn invert(values: &[i32]) -> Vec<i32> {
//    values.iter().map(|x| x * -1).collect()
//  }

fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|x| -x).collect()
}
