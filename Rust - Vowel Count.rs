// Rust - Vowel Count.rs
// https://www.codewars.com/kata/54ff3102c1bad923760001f3

//Incomplete

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    
    let mut vowels_list = vec!["a", "e", "i", "o", "u"];
    
    for i in string.split(""){
        if vowels_list.contains(&i) {
            vowels_count += 1;
        }
    }
    
    vowels_count
}
