///This function checks if strings are rotations of each other

pub fn check_str_rev(input: &str , input2:&str) ->String {
    let mut rev_input = String::new();
    let mut result = String::new();
    for c in input.chars().rev() {
        rev_input.push(c);
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

    #[test]
    fn check_input_string() {
        assert_eq!(check_str_rev("abcd","dcba"), "Matched");
    }
}
