fn main() {
    println!("{}", 1 / 10);
}

fn digits(n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut n = n;
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

// fn digits(mut n: u64) -> usize {
//     let mut l = 1;
//     while n >= 10 {
//         n /= 10;
//         l += 1;
//     }
//     l
// }

#[test]
fn sample_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
