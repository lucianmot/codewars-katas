// Rust - Can we divide it?
// https://www.codewars.com/kata/5a2b703dc5e2845c0900005a/train/rust

//  fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
//     if number % a == 0 && number % b == 0 {true} else {false}
//  }

  fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
     number % a == 0 && number % b == 0
}
