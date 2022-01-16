// Rust - Is he gonna survive?
// https://www.codewars.com/kata/59ca8246d751df55cc00014c/train/rust

//  fn hero(bullets: u16, dragons: u16) -> bool {
//    if bullets / 2 >= dragons { true } else { false }
//  }

fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}
