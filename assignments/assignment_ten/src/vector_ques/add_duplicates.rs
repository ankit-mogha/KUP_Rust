/// duplicate function duplicates the elements of list.
///
/// #Arguments
///
/// vec : its a list of elements vector type.
///
/// #Return
///
/// Returns list of vector type having duplicate values.
pub fn duplicate(vec: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new()
    for index in vec {
        res.push(index);
        res.push(index);
    }
    log::info!("List : {:?}", res);
    res
}
