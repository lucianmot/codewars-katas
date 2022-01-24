// Rust - Bit Counting
// https://www.codewars.com/kata/526571aae218b8ee490006f4/train/rust

fn count_bits(n: i64) -> u32 {
  n.count_ones()
}
