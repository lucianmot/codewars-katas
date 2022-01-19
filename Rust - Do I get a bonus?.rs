// Rust - Do I get a bonus?
// https://www.codewars.com/kata/56f6ad906b88de513f000d96/train/rust

//  fn bonus_time(salary: u64, bonus: bool) -> String {
//    if bonus == true {format!("¥{}", salary * 10)} else {format!("¥{}", salary)}
//  }

fn bonus_time(salary: u64, bonus: bool) -> String {
  format!("¥{}", salary * if bonus {10} else {1})
}
