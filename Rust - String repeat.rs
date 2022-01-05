// Rust - String repeat
// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust

// fn repeat_str(src: &str, count: usize) -> String {
//    let mut results: String = "".to_string();
//    for _ in 0..count {
//        results.push_str(&src)
//    }
//  results
// }

fn repeat_str(src: &str, count: usize) -> String {
  src.repeat(count)
}
