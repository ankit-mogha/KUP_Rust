///This function checks if the string is palindrome or not

pub fn check_palindrome(input: &str) -> String {
    let mut rev_input = String::new();
    let mut result = String::new();

    for char in input.chars().rev() {
        rev_input.push(char);
    }

    if input.eq(&rev_input)==true
    {
        result.push_str("Palindrome");
    }
    else
    {
        result.push_str("Not Palindrome");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    ///Test Case for palindrome string
    #[test]
    fn check_input_string() {
        assert_eq!(check_palindrome("abba"), "Palindrome");
    }

    ///Single character is palindrome string
    #[test]
    fn check_input_1() {
        assert_eq!(check_palindrome("a"), "Palindrome");
    }

    ///Test Case for non palindrome string
    #[test]
    fn check_input_2() {
        assert_eq!(check_palindrome("abbc"), "Not Palindrome");
    }
}
