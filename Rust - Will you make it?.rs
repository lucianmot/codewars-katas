// Rust - Will you make it?
// https://www.codewars.com/kata/5861d28f124b35723e00005e/train/rust

//  fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
//    if mpg * gallons >= distance_to_pump {true} else {false}
//  }

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    mpg * gallons >= distance_to_pump
}
