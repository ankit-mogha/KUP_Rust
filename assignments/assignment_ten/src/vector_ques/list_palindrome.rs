/// is_palindrome function checks if the list is palindrome or not.
///
/// #Arguments
///
/// vec : its a list of elements vector type.
///
/// #Return
///
/// Returns ture if list is palindrome else false if list is not palindrome.
pub fn is_palindrome(vec: Vec<i32>) -> Option<bool> {
    let half_len = vec.len() / 2;
    let len = vec.len();
    let mut count = 0;
    let mut index = 0;
    while index <= half_len && len != 0 {
        if vec[index] != vec[len - index - 1] {
            count = 1;
            break;
        }
        index += 1;
    }
    Some(count != 1)
}
