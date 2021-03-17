pub mod tests {
    #[test]
    fn substring_position_found() {
        use crate::ques1::pattern_search::pattern_check;
        let string = "Ankit Mogha";
        let pattern = "Mog";
        assert_eq!(pattern_check(string, pattern), 6);
    }

    #[test]
    fn substring_position_not_found() {
        use crate::ques1::pattern_search::pattern_check;
        let string = "Ankit Mogha";
        let pattern = "Abc";
        assert_eq!(pattern_check(string, pattern), 0);
    }

    #[test]
    fn check_substring() {
        use crate::ques1::substring_generate::sub_string_find;
        assert_eq!(sub_string_find("pa"), ["p", "pa", "a"]);
    }

    #[test]
    fn check_desired_output_string() {
        use crate::ques2::desired_output::desired_output_fn;
        assert_eq!(desired_output_fn("Ankit", "Mogha", "AnkitMogha"), "Aogia");
    }
}
