// Rust - Reversed sequence
// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust

//  fn reverse_seq(n: u32) -> Vec<u32> {
//    let mut kakat: Vec<u32> = Vec::new();
//    for x in (1..n+1).rev(){
//        kakat.push(x);
//    }
//    return kakat
//  }

fn reverse_seq(n: u32) -> Vec<u32> {
    (1..n+1).rev().collect()
}
