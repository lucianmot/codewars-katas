// Rust - Exes and Ohs
// https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust

fn xo(string: &'static str) -> bool {    
  let mut x :i32 = 0;
  let mut o :i32 = 0;
  
  for c in string.chars() {
      if &c.to_lowercase().to_string() == "x" { x += 1; }
      else if &c.to_lowercase().to_string() == "o" { o += 1; }
  }
  
  return x == o;
}
