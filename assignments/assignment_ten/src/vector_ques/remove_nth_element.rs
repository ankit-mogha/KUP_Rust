/// drop function finds the given number in the list and removes it.
///
/// #Arguments
///
/// vec : its a list of numbers.
///
/// #Return
///
/// Returns list of number of type Vec<i32> after deleting the number.

pub fn drop(mut vec: Vec<i32>, num: i32) -> Option<Vec<i32>> {
    let mut index = 0;
    while index < vec.len() {
        if num == vec[index] {
            vec.remove(index);
        }

        index += 1;
    }
    Some(vec)
}
