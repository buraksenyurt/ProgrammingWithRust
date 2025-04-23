#[allow(dead_code)]
fn min_length(length: usize) -> impl Fn(&str) -> bool {
    move |input| input.len() > length
}

#[allow(dead_code)]
fn is_digits_only() -> impl Fn(&str) -> bool {
    |input| input.chars().all(|c| c.is_ascii_digit())
}

#[allow(dead_code)]
fn illegal_chars(illegals: Vec<char>) -> impl Fn(&str) -> bool {
    move |input| input.chars().any(|c| illegals.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_length_test() {
        let validator = Box::new(min_length(10));
        let input = "Jan Claud Van D@me!";
        assert!(validator(input));
    }

    #[test]
    fn only_digits_test() {
        let validator = Box::new(is_digits_only());
        let input = "12345";
        assert!(validator(input));
    }

    #[test]
    fn illegal_chars_test() {
        let validator = Box::new(illegal_chars(vec!['=', ';']));
        let input = "Select * from Products; 1=1; Select * from sys";
        assert!(validator(input));
    }
}
