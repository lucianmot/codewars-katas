// Rust - Reversed Words
// https://www.codewars.com/kata/51c8991dee245d7ddf00000e/train/rust

//  fn reverse_words(words: &str) -> String {
//    let mut split_words: Vec<_> = words.split_whitespace().collect();
//    split_words.reverse();
//    let joined_words = split_words.connect(" ");
//    joined_words
//  }

fn reverse_words(str: &str) -> String {
   str.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
