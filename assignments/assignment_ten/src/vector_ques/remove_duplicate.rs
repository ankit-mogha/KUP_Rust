/// compress function removes duplicates elements from the list.
///
/// #Arguments
///
/// vec : its a list of elements vector type.
///
/// #Return
///
/// Returns list of vector type after removing duplicate values.
pub fn compress(vec: Vec<i32>) -> Option<Vec<i32>> {
    let mut res: Vec<i32> = Vec::new();
    for value in vec {
        let top = res.last();
        match top {
            Some(e) => {
                if value != *e {
                    res.push(value);
                }
            }
            None => res.push(value),
        }
    }
    log::info!("List : {:?}", res);
    Some(res)
}
