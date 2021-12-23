// Rust - Is a number prime?.rs
// https://www.codewars.com/kata/5262119038c0985a5b00029f/train/rust

fn is_prime(x: i64) -> bool {
//     println!("this is x : {}", x);
    if x > 40 {
        let mut n = 0;
        while (n * n + n + 41) <= x {
            println!("this is n+ 40 : {}      END  ======", (n * n + n + 41));
            n += 1;
        }
//         println!("this is n+ 40 : {}", (n * n + n + 41));
        if (n * n + n + 41) == x {
            return true;
        }
    }
    if x == 2 || x == 3 || x == 5 {
        return true;
    }
    if x <= 1 || x % 2 == 0 {
        return false;
    }
    let mut num = 3;
    while num <= x/2 + 1 {
        if x % num == 0 {
            return false;        
    }
        num += 2;
}
    
    return true;
}
