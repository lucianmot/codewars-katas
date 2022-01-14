// Rust - Beginner Series #2 Clock
// https://www.codewars.com/kata/55f9bca8ecaa9eac7100004a/train/rust

//  fn past(h: i32, m: i32, s: i32) -> i32 {
//    ((h * 60 + m) * 60 + s) * 1000
//  }

fn past(h: i32, m: i32, s: i32) -> i32 {
    h * 3600000 + m * 60000 + s * 1000
}
