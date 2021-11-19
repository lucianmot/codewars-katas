// Rust - Who likes it?
// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/rust

fn likes(names: &[&str]) -> String {
    match names.len() {
        0=>return "no one likes this".to_string(),
        1=>return format!("{} likes this",names[0]),
        2=>return format!("{} and {} like this",names[0], names[1]),
        3=>return format!("{}, {} and {} like this",names[0], names[1], names[2]),
        _=>return format!("{}, {} and {} others like this",names[0], names[1], names.len()-2),
    }
}
