use async_std::task;
use std::time::Duration;
/// async_print_tables function simultaneously print two table of 2 amd 3 in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return value.
pub async fn async_print_tables() {
    env_logger::init();
    let table_of_2 = async {
        for index in 1..11 {
            log::debug!("2 X {} = {}", index, 2 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let table_of_3 = async {
        for index in 1..11 {
            log::debug!("3 X {} = {}", index, 3 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(table_of_2, table_of_3);
}
