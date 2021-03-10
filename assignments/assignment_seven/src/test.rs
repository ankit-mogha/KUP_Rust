pub mod tests {
    #[test]
    fn check_position() {
        use crate::ques1::pattern_search::pattern_check;
        let string = "Ankit Mogha";
        let pattern = "Mog";
        assert_eq!(pattern_check(string, pattern), 6);
    }

    #[test]
    fn check_position_not_found() {
        use crate::ques1::pattern_search::pattern_check;
        let string = "Ankit Mogha";
        let pattern = "Abc";
        assert_eq!(pattern_check(string, pattern), 0);
    }

    #[test]
    fn check_sub() {
        use crate::ques1::substring_generate::sub_string_find;
        assert_eq!(sub_string_find("a"), ['a']);
    }

    #[test]
    fn check_main_fn() {
        use crate::main_fn;
        assert!(main_fn());
    }
}
