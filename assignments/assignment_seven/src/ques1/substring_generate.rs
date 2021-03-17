/// sub_string_find function checks finds substring of given string.
///
/// #Arguments
///
/// str(&str) : string.
///
/// #Return
///
/// Returns Vector having the substring of given string
pub fn sub_string_find(str: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut substr: &str;
    for index in 0..str.len() {
        for inner_index in index..str.len() {
            substr = &str[index..(inner_index + 1)];
            output.push(substr.to_string());
        }
    }
    output
}
