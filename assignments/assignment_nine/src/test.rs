pub mod tests {

    #[test]
    fn check_for_int_values() {
        use crate::sorting_and_compare::compare_two_value::compare_values;
        assert_eq!(compare_values(10, 5), 10);
    }

    #[test]
    fn check_for_float_values() {
        use crate::sorting_and_compare::compare_two_value::compare_values;
        assert_eq!(compare_values(-1.5, 5.9), 5.9);
    }

    #[test]
    fn check_for_int_array() {
        use crate::sorting_and_compare::sorting_array::sort_array;
        let mut arr = [1, 5, 8, 2, 3, 4];
        assert_eq!(sort_array(&mut arr), [1, 2, 3, 4, 5, 8]);
    }

    #[test]
    fn check_for_float_array() {
        use crate::sorting_and_compare::sorting_array::sort_array;
        let mut arr = [1.8, 5.9, 8.3, -2.1, 3.9, 4.5];
        assert_eq!(sort_array(&mut arr), [-2.1, 1.8, 3.9, 4.5, 5.9, 8.3]);
    }

    #[test]
    fn fin_gp_series() {
        use crate::gp_series::geometric_series::CustomIterator;
        use crate::gp_series::geometric_series::GeometricSeries;
        let mut series = GeometricSeries {
            first_number: 2,
            ratio: 3,
            current_number: 0,
        };
        assert_eq!(
            series.gp_gen(),
            [2, 6, 18, 54, 162, 486, 1458, 4374, 13122, 39366, 118098]
        );
    }
}
