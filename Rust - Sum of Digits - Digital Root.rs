// Rust - Sum of Digits - Digital Root
// https://www.codewars.com/kata/541c8630095125aba6000c00/train/rust

fn digital_root(mut n: i64) -> i64 { 
    while n >= 10 {
        let mut sum = 0;
        loop {
            sum += n%10;
            n /= 10;
            if n == 0  {break;}
        }
       n = sum;
    }
    n
}
