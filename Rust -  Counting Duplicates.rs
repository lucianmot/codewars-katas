// Rust -  Counting Duplicates
// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust

// Incomplete Solution

use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    println!("{}", text);
    let mut storage: HashMap<&char, u32> = HashMap::new();
    for c in text.chars() {
        {
        let _pokemon = storage.entry(&c).or_insert(1);
//             storage.insert(String::from(c), 0);
        }
        println!("{:?}", storage);
    }
    
    return 1;
}
