// Rust - Remove First and Last Character
// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust

// pub fn remove_char(s: &str) -> String {
//    let mut remainder = s.to_string();
//    remainder.pop();
//    remainder.remove(0);
//    remainder
// }

pub fn remove_char(s: &str) -> String {
    s[1..s.len()-1].into()
}
