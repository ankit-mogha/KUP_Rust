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

use crate::list_ques::second_repeated_num::List::{Cons, Nil};
/// second_repeated function gives the second repeated number of the given list.
///
/// #Arguments
///
/// 'prev_number' - An i32 variable containing the previous value of the list.
///
/// 'list' - Contains the list of numbers.
///
/// 'counter' - a variable that keeps the count of the function call.
///
/// #Return
///
/// Return the number containing second repeated number of the given list.
pub fn second_repeated(counter: i32, prev_number: i32, list: List) -> Option<i32> {
    match list {
        Nil => Some(-1),
        Cons(current_num, list) => {
            if prev_number == current_num && counter == 0 {
                second_repeated(counter + 1, current_num, *list)
            } else if prev_number == current_num && counter == 1 {
                Some(current_num)
            } else {
                second_repeated(counter, current_num, *list)
            }
        }
    }
}
