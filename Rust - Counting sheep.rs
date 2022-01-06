// Rust - Counting sheep...
// https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust
// INCOMPLETE

fn count_sheep(sheep: &[bool]) -> u8 {
//     let mut shit = sheep.retain(|&x| x == true);
//     println!("{}", shit);
//     return 1;
    if let Some(pos) = sheep.iter().position(|x| *x != true) {
        sheep.remove(pos);
    }
    println!("{:?}", sheep);
    return 1
}
