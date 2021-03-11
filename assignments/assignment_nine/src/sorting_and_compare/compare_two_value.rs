/// compare_values function compare two values.
///
/// #Arguments
///
/// num1 : First number.
/// num2 : Second number.
///
/// #Return
///
/// Returns a greatest vale from given two values
pub fn compare_values<T: PartialOrd>(num1: T, num2: T) -> T {
    match num1 >= num2 {
        true => num1,
        false => num2,
    }
}
