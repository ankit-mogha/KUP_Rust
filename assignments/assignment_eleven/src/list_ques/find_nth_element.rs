///List Enum.
///
/// #Variants.
///
/// Cons : construct function.
///
/// Nil.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::list_ques::find_nth_element::List::{Cons, Nil};
/// nth_number_finder function find the number present at given index.
///
/// #Arguments
///
/// 'index' - An i32 variable containing the position of number to find in list.
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// 'counter' - An i32 variable containing the position of current number in list.
///
/// #Return
///
/// Return the number at the given index.
pub fn nth_number_finder(index: i32, counter: i32, list: List) -> Option<i32> {
    match list {
        Nil => Some(-1),
        Cons(number, list) => {
            if index == counter {
                Some(number)
            } else {
                nth_number_finder(index, counter + 1, *list)
            }
        }
    }
}
