// Rust - Century From Year
// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/train/rust

//  fn century(year: u32) -> u32 {
//    if year % 100 == 0 { year / 100 } else { year / 100 + 1 }
//  }
//  fn century(year: u32) -> u32 {
//    (year as f32 / 100.).ceil() as u32
//  }
fn century(year: u32) -> u32 {
    (year + 99) / 100
}
