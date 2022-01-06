// Rust - Remove String Spaces
// https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust

//  fn no_space(x : String) -> String {
//    x.split_whitespace().collect()
//  }

fn no_space(x : String) -> String{
    x.replace(" ", "")
}
