#[cfg(test)]
pub mod tests {
    use crate::async_await::async_ques::async_fn;
    use crate::async_await::tables_asynchronously::async_print_tables;
    use futures::executor::block_on;

    #[test]
    fn check_async_print_of_tables() {
        assert_eq!(block_on(async_print_tables()), ());
    }

    #[test]
    fn check_async_fn() {
        assert_eq!(block_on(async_fn()), ());
    }
}
