pub fn find_rep_char(str: &str) -> String{
    let mut my_vec: Vec<char> = str.chars().collect();
    let mut i = 0;
    let mut result = String::new();
    while i < str.len() {
        let mut count = 1;
       let mut j = i + 1;
        while j < str.len() {
            if my_vec[i] == my_vec[j] && my_vec[i] != ' ' {
                count += 1;
                my_vec[j] = '0';
            }
            j += 1;
        }
        if count > 1 && my_vec[i] != '0' {
            result.push(my_vec[i]);
        }
        i += 1;
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
}

