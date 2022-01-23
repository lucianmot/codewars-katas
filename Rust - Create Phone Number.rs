// Rust - Create Phone Number
// https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust

//  fn create_phone_number(numbers: &[u8]) -> String {
//    format!("({}{}{}) {}{}{}-{}{}{}{}", numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7], numbers[8], numbers[9])
//  }

fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.into_iter().map(|x| x.to_string()).collect();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}
