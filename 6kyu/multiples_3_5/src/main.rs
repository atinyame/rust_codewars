fn main() {}

fn solution(num: i32) -> i32 {
    (0..num).filter(|x| x % 5 == 0 || x % 3 == 0).sum()
}

#[test]
fn returns_expected() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(11), 33);
    assert_eq!(solution(6), 8);
}
