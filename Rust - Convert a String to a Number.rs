// Rust - Convert a String to a Number!
// https://www.codewars.com/kata/544675c6f971f7399a000e79/train/rust

//  fn string_to_number(s: &str) -> i32 {
//     s.parse::<i32>().unwrap()    
//  }

fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()    
}
