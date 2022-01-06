// Rust - Square(n) Sum
// https://www.codewars.com/kata/515e271a311df0350d00000f/train/rust

//  fn square_sum(vec: Vec<i32>) -> i32 {
//    let mut results: i32 = 0;
//    for i in vec.iter() {
//        results = results + i.pow(2);
//    }
//    return results;
//  }

//  fn square_sum(vec: Vec<i32>) -> i32 {
//    vec.iter().map(|x| x.pow(2)).sum::<i32>()
//  }

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x * x).sum()
}
