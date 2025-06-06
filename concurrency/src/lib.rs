mod asynch;
mod threads;

pub fn concurrency_examples() {
    threads();
    asynch_await();
}

fn threads() {
    threads::spawn_one_thread();
    threads::spawn_multiple_threads(10, 1000);
    threads::message_passing_with_mpsc(10, 1000);
    threads::state_sharing_with_mutex();
}

fn asynch_await() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        asynch::demonstrate_async().await;
        asynch::demonstrate_blocking().await;
    });
}
