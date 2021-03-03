///This Function tells the repetitive characters in string

pub fn find_rep_char(str: &str) -> String{
    let mut my_str: Vec<char> = str.chars().collect();
    let mut counter = 0;
    let mut result = String::new();

    while counter < str.len() {
        let mut count = 1;
        let mut inner_counter = counter + 1;
        while inner_counter < str.len() {
            if my_str[counter] == my_str[inner_counter] && my_str[counter] != ' ' {
                count += 1;
                my_str[inner_counter] = '0';
            }
            inner_counter += 1;
        }
        if count > 1 && my_str[counter] != '0' {
            result.push(my_str[counter]);
        }
        counter += 1;
    }
    return result
}

/*pub fn find_rep_char(str: &str, i: usize, j: usize) {
    let mut my_vec: Vec<char> = str.chars().collect();
    if i < str.len() {
        let mut count = 1;
        if j < str.len() {
            if my_vec[i] == my_vec[j] && my_vec[i] != ' ' {
                count += 1;
                my_vec[j] = '0';
            }
            let j = j + 1;
            find_rep_char(&str, i, j);
        }
        if count > 1 && my_vec[i] != '0' {
            print!("{} ", my_vec[i]);
        }
        let i = i + 1;
        find_rep_char(&str, i, j);
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_func() {
        assert_eq!(find_rep_char("Hello World"), "lo");
    }

    #[test]
    fn check_func_1() {
        assert_eq!(find_rep_char("Hello"), "l");
    }
    /// Test case when no character is repetitive
    #[test]
    fn check_func_2() {
        assert_eq!(find_rep_char("ankit"), "");
    }
}

