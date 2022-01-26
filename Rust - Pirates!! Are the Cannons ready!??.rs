// Rust - Pirates!! Are the Cannons ready!??
// https://www.codewars.com/kata/5748a883eb737cab000022a6/train/rust

use std::collections::HashMap;

fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    for (k, v) in gunners {
        if v == "nay" {
            return "Shiver me timbers!".to_owned();
        }
    }
    "Fire!".to_owned()
}
