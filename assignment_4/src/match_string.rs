/// check_str_rev function checks if strings are rotations of each other
///
/// #Arguments
///
/// input : First String
/// input2 : Second String
///
/// #Return
///
/// this function return a string

pub fn check_str_rev(input: &str, input2: &str) -> String {
    let mut rev_input = String::new();
    let mut result = String::new();

    for char in input.chars().rev() {
        rev_input.push(char);
    }

    if input2.eq(&rev_input) == true {
        result.push_str("Matched");
    } else {
        result.push_str("Did not Matched");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_positive_test_case() {
        assert_eq!(check_str_rev("abcd", "dcba"), "Matched");
    }

    #[test]
    fn check_negative_test_case() {
        assert_eq!(check_str_rev("abc", "dcb"), "Did not Matched");
    }
}
