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

use crate::list_ques::third_odd_number::List::{Cons, Nil};
/// third_odd function gives the third odd number present in the given list.
///
/// #Arguments
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// 'counter' - An i32 variable containing the position of current number of the list.
///
/// #Return
///
/// Return the third odd number of the given list.
pub fn third_odd(counter: i32, list: List) -> i32 {
    match list {
        Nil => -1,
        Cons(num, list) => {
            let result = num % 2 != 0;
            match result {
                true => {
                    if counter == 1 || counter == 0 {
                        third_odd(counter + 1, *list)
                    } else {
                        num
                    }
                }
                false => third_odd(counter, *list),
            }
        }
    }
}
