// Rust - Find the next perfect square!
// https://www.codewars.com/kata/56269eb78ad2e4ced1000013/train/rust

fn find_next_square(sq: u64) -> Option<u64> {    
  let n = (sq as f64).sqrt();
  if n != n.trunc() { None } else {Some (((n as u64 + 1) * (n as u64 + 1)))}
}
