///This function checks if strings are rotations of each other

pub fn check_str_rev(input: &str , input2:&str) ->String {
    let mut rev_input = String::new();
    let mut result = String::new();

    for char in input.chars().rev() {
        rev_input.push(char);
    }

    if input2.eq(&rev_input)==true
    {
        result.push_str("Matched");
    }

    else {
        result.push_str("Did not Matched");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    ///Test Case when string is reverse of another one
    #[test]
    fn check_input_string() {
        assert_eq!(check_str_rev("abcd","dcba"), "Matched");
    }

    ///Test Case when string is not reverse of another one
    #[test]
    fn check_input() {
        assert_eq!(check_str_rev("abc","dcb"), "Did not Matched");
    }
}
