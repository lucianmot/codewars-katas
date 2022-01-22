// Rust - Count of positives and sum of negatives
// https://www.codewars.com/kata/576bb71bbbcf0951d5000044/train/rust

//  fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
//    if input == [] {
//        return vec![]
//    }
//    let mut count_positive = 0;
//    let mut sum_negative = 0;
//    for x in input.iter() {
//        println!("{}", x);
//        if x > &0 {
//            count_positive += 1;
//        } else {
//            sum_negative += x;
//        }
//    }
//    return vec![count_positive, sum_negative]
//  }

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
      if input == [] {
        return vec![]
    }
        input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 {
        acc[0] += 1;
      } else {
        acc[1] += x;
      }
      acc
    })
}
