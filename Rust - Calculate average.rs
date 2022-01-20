// Rust - Calculate average
// https://www.codewars.com/kata/57a2013acf1fa5bfc4000921/train/rust

//  fn find_average(slice: &[f64]) -> f64 {
//    if slice.is_empty() {0.0} else {slice.iter().sum::<f64>() / slice.len() as f64}
//  }

fn find_average(slice: &[f64]) -> f64 {
    if slice == [] {0.0} else {slice.iter().sum::<f64>() / slice.len() as f64}
}
