/// sort_array function sort the given array.
///
/// #Arguments
///
/// arr : array of generic type numbers .
///
/// #Return
///
/// Returns a sorted array
pub fn sort_array<T: PartialOrd + Copy>(arr: &mut [T]) -> &[T] {
    for index in 0..arr.len() {
        for inner_index in (index + 1)..arr.len() {
            let temp;
            if arr[index] > arr[inner_index] {
                temp = arr[index];
                arr[index] = arr[inner_index];
                arr[inner_index] = temp;
            }
        }
    }
    arr
}
