#[cfg(test)]
pub mod tests {
    use crate::hash_map_ques::hash_map::sum_conditional;
    use crate::vector_ques::first_even_number::first_even;
    use crate::vector_ques::remove_nth_element::drop;
    use std::collections::HashMap;

    #[test]
    fn check_when_pattern_present_in_key() {
        let mut map = HashMap::new();
        map.insert(String::from("anurag"), 24);
        map.insert(String::from("daniel"), 23);
        map.insert(String::from("anushka"), 30);
        assert_eq!(sum_conditional(&map, "anu".to_string()), Some(54));
    }

    #[test]
    fn check_when_pattern_not_present_in_key() {
        let mut map = HashMap::new();
        map.insert(String::from("anurag"), 24);
        map.insert(String::from("daniel"), 50);
        map.insert(String::from("anushka"), 30);
        assert_eq!(sum_conditional(&map, "kip".to_string()), Some(0));
    }

    #[test]
    fn check_single_even_number() {
        let vec = vec![1, 21, 3, 4, 5];
        assert_eq!(first_even(vec), Some(4));
    }

    #[test]
    fn check_more_than_one_even_number() {
        let vec = vec![10, 21, 3, 4, 5];
        assert_eq!(first_even(vec), Some(10));
    }

    #[test]
    fn check_when_number_drop() {
        let vec = vec![10, 21, 3, 4, 5];
        assert_eq!(drop(vec, 4), Some(vec![10, 21, 3, 5]));
    }

    #[test]
    fn check_when_number_not_in_list() {
        let vec = vec![10, 21, 3, 4, 5];
        assert_eq!(drop(vec, 19), Some(vec![10, 21, 3, 4, 5]));
    }

    #[test]
    fn check_add_duplicate() {
        use crate::vector_ques::add_duplicates::duplicate;
        let vec = vec![10, 21, 3, 4, 5];
        assert_eq!(duplicate(vec), Some(vec![10, 10, 21, 21, 3, 3, 4, 4, 5, 5]));
    }

    #[test]
    fn check_remove_duplicate() {
        use crate::vector_ques::remove_duplicate::compress;
        let vec = vec![1, 1, 1, 1, 2, 3, 3, 1, 1, 4, 5, 5, 5, 5];
        assert_eq!(compress(vec), Some(vec![1, 2, 3, 1, 4, 5]));
    }

    #[test]
    fn check_reverse_list() {
        use crate::vector_ques::reverse_list::reverse;
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(reverse(vec), Some(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn check_list_palindrome() {
        use crate::vector_ques::list_palindrome::is_palindrome;
        let vec = vec![1, 2, 3, 2, 1];
        assert_eq!(is_palindrome(vec), Some(true));
    }

    #[test]
    fn check_list_not_palindrome() {
        use crate::vector_ques::list_palindrome::is_palindrome;
        let vec = vec![1, 2, 3];
        assert_eq!(is_palindrome(vec), Some(false));
    }
}
