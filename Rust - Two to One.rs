// Rust - Two to One
// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust

fn longest(a1: &str, a2: &str) -> String {    
  let mut l :String = a1.to_string() + a2;
  let mut v :Vec<_> = l.chars().collect();
  
  v.sort();
  v.dedup();
  
  return v.iter().collect::<String>();
}
