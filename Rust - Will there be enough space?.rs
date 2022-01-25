// Rust - Will there be enough space?
// https://www.codewars.com/kata/5875b200d520904a04000003/train/rust

//  fn enough(cap: i32, on: i32, wait: i32) -> i32 {
//    if (cap - on - wait) < 0 {on + wait - cap} else {0}
//  }

fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    (on + wait - cap).max(0)
}
