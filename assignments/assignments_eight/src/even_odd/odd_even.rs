/// check_num function checks number and handles Result enum.
///
/// #Arguments
///
/// num(i32) : number passed from main_fn function.
///
/// #Return
///
/// Returns String
pub fn check_num(num: i32) -> String {
    let result = check(num);
    match result {
        Ok(num) => num,

        Err(n) => n,
    }
}
/// check function checks number is even or not.
///
/// #Arguments
///
/// num(i32) : number that need to be check .
///
/// #Return
///
/// Returns Result<String, String> type Enum
fn check(num: i32) -> Result<String, String> {
    if num % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Odd".to_string())
    }
}
