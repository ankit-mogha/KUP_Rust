pub mod tests {
    #[test]
    fn check_even_num() {
        use crate::odd_even_ques::odd_even_checker::check_output;
        let num = 10;
        assert_eq!(check_output(num), "Even");
    }

    #[test]
    fn check_odd_num() {
        use crate::odd_even_ques::odd_even_checker::check_output;
        let num = 13;
        assert_eq!(check_output(num), "Odd");
    }
}
