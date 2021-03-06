// Rust - Switch it Up!
// https://www.codewars.com/kata/5808dcb8f0ed42ae34000031/train/rust

//  fn switch_it_up(n: usize) -> &'static str {
//    match n {
//        0 => "Zero",
//        1 => "One",
//        2 => "Two",
//        3 => "Three",
//        4 => "Four",
//        5 => "Five",
//        6 => "Six",
//        7 => "Seven",
//        8 => "Eight",
//        _ => "Nine",
//    }
//  }

fn switch_it_up(n: usize) -> &'static str {
    ["Zero","One","Two","Three","Four","Five","Six","Seven","Eight","Nine"][n]
}
