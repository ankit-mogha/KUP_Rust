use async_std::task;
use std::time::Duration;
/// async_fn function simultaneously to runs two task asynchronous block.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return value.
pub async fn async_fn() {
    let async_block_1 = async {
        for i in 1..3 {
            log::debug!("{}.async block 1", i);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let async_block_2 = async {
        for i in 1..3 {
            log::debug!("{}.async block 2", i);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(async_block_1, async_block_2);
}
