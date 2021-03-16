/// reverse function reverse the list.
///
/// #Arguments
///
/// vec : its a list of elements vector type.
///
/// #Return
///
/// Returns list of vector type after reversing them.
pub fn reverse(mut vec: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut len = vec.len();
    while len > 0 {
        let last = vec.last();
        if let Some(ele) = last {
            res.push(*ele);
            vec.pop();
        }
        len -= 1;
    }
    log::info!("List : {:?}", res);
    res
}
