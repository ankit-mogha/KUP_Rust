///pattern_check function checks for pattern in given string.
///
/// #Arguments
///
/// string : given string from which pattern need to be searched.
/// pattern : pattern that need to be searched from given string.
///
/// #Return
///
/// Returns index(usize) of the starting point of pattern
pub fn pattern_check(string: &str, pattern: &str) -> usize {
    let len_pattern = pattern.len();
    let len_string = string.len();
    let string_vec: Vec<char> = string.chars().collect();
    let pattern_vec: Vec<char> = pattern.chars().collect();
    let mut index = 0;
    while index <= len_string - len_pattern {
        let mut inner_index = 0;
        while inner_index < len_pattern {
            if string_vec[index + inner_index] != pattern_vec[inner_index] {
                break;
            }
            inner_index += 1;
        }
        if inner_index == len_pattern {
            return index;
        }
        index += 1;
    }
    0
}
