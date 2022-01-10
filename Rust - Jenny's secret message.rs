// Rust - Jenny's secret message
// https://www.codewars.com/kata/55225023e1be1ec8bc000390/train/rust

//  fn greet(input : &str) -> String {
//    if input == "Johnny" {
//        return "Hello, my love!".to_string();
//  } else {
//      return format!("Hello, {}!", input);
//    }
//  }

fn greet(input : &str) -> String {
  match input {
    "Johnny" => "Hello, my love!".to_string(),
         _   => format!("Hello, {}!", input),
  }
}
