pub mod tests {

    #[test]
    fn check_even_num() {
        use crate::even_odd::odd_even::check_num;
        assert_eq!(check_num(10), "Even");
    }

    #[test]
    fn check_odd_num() {
        use crate::even_odd::odd_even::check_num;
        assert_eq!(check_num(13), "Odd");
    }

    #[test]
    fn check_main_fn() {
        use crate::main_fn;
        assert!(main_fn());
    }
}
