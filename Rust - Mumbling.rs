// Rust - Mumbling
// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust

fn accum(s:&str)->String {
    
  let mut result = String::new();
  let mut index :usize = 0;
  
  for c in s.chars() {
      index += 1;
      
      result.push_str(&c.to_uppercase().to_string());
      
      for k in 2..=index {
          result.push_str(&c.to_lowercase().to_string());
      }
      
      result.push('-');
  }
  
  result.remove(result.len()-1);  
  return result;
}
