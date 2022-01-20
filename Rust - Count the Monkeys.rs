// Rust - Count the Monkeys!
// https://www.codewars.com/kata/56f69d9f9400f508fb000ba7/train/rust

//  fn monkey_count(n: i32) -> Vec<i32> {
//    let mut vecky_boy: Vec<i32> = Vec::new();
//    for x in 1..n+1 {
//        vecky_boy.push(x);
//    }
//    return vecky_boy
//  }

fn monkey_count(n: i32) -> Vec<i32> {
    (1..n+1).collect()
}
