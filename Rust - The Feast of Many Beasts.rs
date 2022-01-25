// Rust - The Feast of Many Beasts
// https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust

//  fn feast(beast: &str, dish: &str) -> bool {
//    if beast.chars().last().unwrap() == dish.chars().last().unwrap() && beast.chars().next().unwrap() == dish.chars().next().unwrap() {true} else {false}
//  }

fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() && beast.chars().last() == dish.chars().last()
}
