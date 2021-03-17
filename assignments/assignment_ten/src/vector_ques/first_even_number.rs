/// first_even function checks first even number in the list .
///
/// #Arguments
///
/// vec : its a list of numbers.
///
/// #Return
///
/// Returns i32 that is first even number in list.

pub fn first_even(vec: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut index = 0;
    while index < vec.len() {
        if vec[index] % 2 == 0 {
            count += 1;
            if count < 2 {
                return vec[index];
            }
        }
        index += 1;
    }
    0
}
