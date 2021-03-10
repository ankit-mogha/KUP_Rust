///check_palindrome function checks if the string is palindrome or not
///
/// #Arguments
///
/// function takes a string as an input
///
/// #Return
///
/// function return a string

pub fn check_palindrome(input: &str) -> String {
    let mut rev_input = String::new();
    let mut result = String::new();

    for char in input.chars().rev() {
        rev_input.push(char);
    }

    if input.eq(&rev_input) == true {
        result.push_str("Palindrome");
    } else {
        result.push_str("Not Palindrome");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_input_string() {
        assert_eq!(check_palindrome("abba"), "Palindrome");
    }

    #[test]
    fn check_a_single_char() {
        assert_eq!(check_palindrome("a"), "Palindrome");
    }

    #[test]
    fn negative_test_case() {
        assert_eq!(check_palindrome("abbc"), "Not Palindrome");
    }
}
