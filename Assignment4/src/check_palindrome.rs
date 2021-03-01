///This function checks if the string is palindrome or not

pub fn check_palindrome(input: &str) -> String {
    let mut rev_input = String::new();
    let mut result = String::new();

    for c in input.chars().rev() {
        rev_input.push(c);
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

    #[test]
    fn check_input_string() {
        assert_eq!(check_palindrome("abba"), "Palindrome");
    }
}