fn main() {}

fn printer_error(s: &str) -> String {
    format!(
        "{}/{}",
        s.chars().filter(|c| !('a'..='m').contains(c)).count(),
        s.len()
    )
}

// fn printer_error(s: &str) -> String {
//     // Your cude here
//     format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(
            &printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "6/60"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"),
            "11/65"
        );
    }
}
