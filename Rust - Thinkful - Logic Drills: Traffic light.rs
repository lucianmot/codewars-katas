// Rust - Thinkful - Logic Drills: Traffic light
// https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust

//  fn update_light(current: &str) -> String {
//    match current {
//        "green" => "yellow",
//        "yellow" => "red",
//        _ => "green",
//    }.to_string()
//    or
//    }.into()
// }

fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        _ => "green",
    }.to_owned()
}
