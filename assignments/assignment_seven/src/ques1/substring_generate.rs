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
    let mut i = 1;
    let mut j = 0;
    while i <= len {
        while j <= len - i {
            let k = j + i - 1;
            let mut z = j;
            while z <= k {
                output.push(my_str[z]);
                z += 1;
            }
            j += 1;
        }
        i += 1;
    }
    output
}
