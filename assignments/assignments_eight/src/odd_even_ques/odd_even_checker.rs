/// check function checks number is even or not.
///
/// #Arguments
///
/// num(i32) : number that need to be checked.
///
/// #Return
///
/// Returns Result<String, String> type Enum.
pub fn check_num(num: i32) -> Result<String, String> {
    if num % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Odd".to_string())
    }
}

/// check_output handles Result of check_num function .
///
/// #Arguments
///
/// num(i32) : number.
///
/// #Return
///
/// Returns String which tell number is even or odd.
pub fn check_output(num: i32) -> String {
    let result = check_num(num);
    match result {
        Ok(even) => even,

        Err(odd) => odd,
    }
}
