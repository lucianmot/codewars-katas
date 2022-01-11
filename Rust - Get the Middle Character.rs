// Rust - Get the Middle Character
// https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust

//  fn descending_order(x: u64) -> u64 {
//    let mut digits: Vec<_> = x.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
//    digits.sort();
//    digits.reverse();
//    let mut results: String = digits.iter().map( |&id| id.to_string()).collect(); 
//    return results.parse::<u64>().unwrap();
//  }

//  fn descending_order(x: u64) -> u64 {
//    let mut digits: Vec<_> = x.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
//    digits.sort_by(|a, b| b.cmp(a));
//    let mut results: String = digits.iter().map( |&id| id.to_string()).collect(); 
//    return results.parse::<u64>().unwrap();
//  }

fn descending_order(x: u64) -> u64 {
    let mut chars: Vec<char> = x.to_string().chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect::<String>().parse().unwrap()
}
