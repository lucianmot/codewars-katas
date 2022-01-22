// Rust - Beginner - Reduce but Grow
// https://www.codewars.com/kata/57f780909f7e8e3183000078/train/rust

//  fn grow(nums: Vec<i32>) -> i32 {
//    let mut results: i32 = 1;
//    for x in nums.iter(){
//        results *= x;
//    }    
//    results
//  }

fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}
