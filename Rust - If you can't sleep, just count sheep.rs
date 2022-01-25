// Rust - If you can't sleep, just count sheep!!
// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust

//  fn count_sheep(n: u32) -> String {
//    let mut sheepo: String = "".to_string();
//    for x in 1..n+1 {
//        let sheep_add: String = format!("{} sheep...", x);
//        sheepo.push_str(&sheep_add);
//    }
//    if n == 0 {"".to_string()} else {sheepo.to_string()}
//  }

fn count_sheep(n: u32) -> String {
    (1..=n).map(|x| format!("{} sheep...", x)).collect()
}
