// Rust - Get the mean of an array
// https://www.codewars.com/kata/563e320cee5dddcf77000158/train/rust

//  fn get_average(marks: &[f32]) -> f32 {
//    (marks.iter().sum::<f32>() as f32 / marks.len() as f32).floor()
//  }

fn get_average(marks: &[f32]) -> f32 {
    (marks.iter().sum::<f32>() / marks.len() as f32).floor()
}
