// Rust - Multiples of 3 or 5
// https://www.codewars.com/kata/514b92a657cdc65150000006/solutions/rust

//  fn solution(num: i32) -> i32 {
//    let mut i = 0;
//    let mut sum = 0;
//
//    while i < num {       
//        if i % 3 == 0 || i % 5 == 0 {
//            sum = sum + i;
//        }
//        i = i + 1;
//        }
//    sum
//  }

//  fn solution(num: i32) -> i32 {
//    let mut results: Vec<i32> = Vec::new();
//    for x in 3..num {
//        if x % 3 == 0 {
//            results.push(x);
//        } else if x % 5 == 0 {
//            results.push(x);
//        }
//    }
//    results.iter().sum()
//  }

fn solution(num: i32) -> i32 {
  (1..num).filter(|x| x % 3 == 0 || x % 5 == 0 ).sum()
}
