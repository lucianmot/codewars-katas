// Rust - Directions Reduction
// https://www.codewars.com/kata/550f22f4d758534c1100025a/train/rust

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
        let mut new_arr: Vec<Direction> = arr.to_vec();
//     for (i, e) in arr.iter().enumerate() {
//         println!("i: {:?}, element: {:?}", i, e);
//         println!("this is current: {:?}, this is next: {:?}", arr[i], arr[i+1]);
//     }
        for i in 0..new_arr.len()-1 {
            println!("this is element: {:?}, this is the next element: {:?}", arr[i], arr[i+1]);
            println!("this is index: {}", i);
            match (arr[i], arr[i+1]) {
                (Direction::North, Direction::South) => {
                    new_arr.remove(i);
                    new_arr.remove(i);
                    i -= 2;
                },
                (Direction::South, Direction::North) => new_arr.remove(i),
                (Direction::West, Direction::East) => new_arr.remove(i),
                (Direction::East, Direction::West) => new_arr.remove(i),
            }
    }
//     println!("{:?}", arr);
    vec![]
}
  
// INCOMPLETE
