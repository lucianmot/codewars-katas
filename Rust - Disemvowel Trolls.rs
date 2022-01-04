// Rust - Disemvowel Trolls
// https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust

fn disemvowel(s: &str) -> String {
    
    s.replace(&['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'][..], "");
    
//     let dic = &['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
//     let mut result = String::new();
    
//     for c in s.chars() {
//         if !dic.contains(&c) {
//            result.push(c);
//         }
//     }
    
//     return result;
    
}
