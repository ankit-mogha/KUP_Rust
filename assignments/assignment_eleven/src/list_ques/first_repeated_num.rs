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

use crate::list_ques::first_repeated_num::List::{Cons, Nil};
/// second_repeated function gives the first repeated number of the given list.
///
/// #Arguments
///
/// 'prev_number' - An i32 variable containing the previous value of the list.
///
/// 'list' - Contains the list of numbers.
///
/// #Return
///
/// Return the number containing first repeated number of the given list.
pub fn first_repeated(prev_number: i32, list: List) -> i32 {
    match list {
        Nil => -1,
        Cons(current_num, list) => match prev_number == current_num {
            true => current_num,
            false => first_repeated(current_num, *list),
        },
    }
}
