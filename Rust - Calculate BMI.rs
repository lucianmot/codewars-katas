// Rust - Calculate BMI
// https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust

//  fn bmi(weight: u32, height: f32) -> &'static str {
//    let bmi = weight as f32 / (height * height);
//    match bmi {
//        0.0..=18.5 => "Underweight",
//        18.6..=25.0 => "Normal",
//        25.1..=30.0 => "Overweight",
//        _ => "Obese",
//    }
//  }

fn bmi(weight: u32, height: f32) -> &'static str {
    match (weight as f32 / (height * height)) {
        0.0..=18.5 => "Underweight",
        18.6..=25.0 => "Normal",
        25.1..=30.0 => "Overweight",
        _ => "Obese",
    }
}
