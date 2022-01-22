// Rust - Get Planet Name By ID
// https://www.codewars.com/kata/515e188a311df01cba000003/train/rust

//  fn get_planet_name(id: u32) -> String {
//    match id {
//        1 => "Mercury".to_string(),
//        2 => "Venus".to_string(),
//        3 => "Earth".to_string(),
//        4 => "Mars".to_string(),
//        5 => "Jupiter".to_string(),
//        6 => "Saturn".to_string(),
//        7 => "Uranus".to_string(),
//        8 => "Neptune".to_string(),
//        _ => unreachable!(),
//    }
//  }

fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury",
        2 => "Venus",
        3 => "Earth",
        4 => "Mars",
        5 => "Jupiter",
        6 => "Saturn",
        7 => "Uranus",
        8 => "Neptune",
        _ => unreachable!(),
    }.to_string()
}
