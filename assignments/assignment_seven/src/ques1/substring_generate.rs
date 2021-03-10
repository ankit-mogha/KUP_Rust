/// sub_string_find function checks finds substring of given string.
///
/// #Arguments
///
/// s(&str) : string.
///
/// #Return
///
/// Returns Vector having the substring od given string
pub fn sub_string_find(s:&str) -> Vec<char>{
    let len = s.len();
    let my_str: Vec<char> = s.chars().collect();
    let mut output: Vec<char> = Vec::new();
    let mut i = 1;
    let mut j = 0;
    while i <= len {
        while j <= len - i{
            let k = j + i - 1;
            let mut z = j;
            while z <= k {
                output.push(my_str[z]);
                z +=1;
            }
            j +=1;
        }
        i+=1;
    }
    output
}

