// Rust - L1: Set Alarm
// https://www.codewars.com/kata/568dcc3c7f12767a62000038/train/rust

//  fn set_alarm(employed: bool, vacation: bool) -> bool {
//    if employed == true && vacation == false {true} else {false}
//  }

fn set_alarm(employed: bool, vacation: bool) -> bool {
    employed & !vacation
}
