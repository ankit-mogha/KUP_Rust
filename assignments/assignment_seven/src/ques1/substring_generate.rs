/// sub_string_find function checks finds substring of given string.
///
/// #Arguments
///
/// str(&str) : string.
///
/// #Return
///
/// Returns Vector having the substring od given string
pub fn sub_string_find(str: &str) -> Vec<char> {
    let len = str.len();
    let my_str: Vec<char> = str.chars().collect();
    let mut output: Vec<char> = Vec::new();
    let mut index = 1;
    let mut inner_index = 0;
    while index <= len {
        while inner_index <= len - index {
            let key_value = inner_index + index - 1;
            let mut sub_char_index = inner_index;
            while sub_char_index <= key_value {
                output.push(my_str[sub_char_index]);
                sub_char_index += 1;
            }
            inner_index += 1;
        }
        index += 1;
    }
    output
}
