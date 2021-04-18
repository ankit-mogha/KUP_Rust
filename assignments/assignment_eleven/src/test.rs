#[cfg(test)]
mod tests {
    use crate::list_ques::find_nth_element::nth_number_finder;
    use crate::list_ques::first_repeated_num::first_repeated;
    use crate::list_ques::first_repeated_num::List::{Cons, Nil};
    use crate::list_ques::second_repeated_num::second_repeated;
    use crate::list_ques::third_odd_number::third_odd;

    #[test]
    fn first_repeat_finder_success() {
        env_logger::init();
        log::info!("finds first repeated number");
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_repeated(0, test_list), Some(21));
    }
    #[test]
    fn second_repeat_finder_success() {
        use crate::list_ques::second_repeated_num::List::{Cons, Nil};
        log::info!("finds Second repeated number");
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(second_repeated(0, 0, test_list), Some(5));
    }
    #[test]
    fn find_number() {
        use crate::list_ques::find_nth_element::List::{Cons, Nil};
        log::info!("finds nth number");
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    21,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(nth_number_finder(3, 0, test_list), Some(4));
    }
    #[test]
    fn find_odd_number() {
        use crate::list_ques::third_odd_number::List::{Cons, Nil};
        log::info!("finds third odd number");
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(third_odd(0, test_list), Some(3));
    }
}
