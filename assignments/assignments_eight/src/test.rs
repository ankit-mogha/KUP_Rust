pub mod tests {

    #[test]
    fn check_even_num() {
        use crate::odd_even::check_num;
        let num = 10;
        assert_eq!(check_num(num), "Even");
    }

    #[test]
    fn check_odd_num() {
        use crate::odd_even::check_num;
        let num = 13;
        assert_eq!(check_num(num), "Odd");
    }

    #[test]
    fn check_main_fn() {
        use crate::main_fn;
        assert!(main_fn());
    }
}
