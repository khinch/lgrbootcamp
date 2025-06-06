use futures::future::select_all;
use tokio::time::{Duration, sleep};

async fn fetch_data_async(source: &str, delay: u64) -> String {
    sleep(Duration::from_millis(delay)).await; // Simulate network delay
    format!("Data from {}", source)
}

fn fetch_data_sync(source: &str, delay: u64) -> String {
    std::thread::sleep(Duration::from_millis(delay)); // Simulate network delay
    format!("Data from {}", source)
}

pub async fn demonstrate_async() {
    println!("Starting async operations...");

    let mut handles = vec![];
    let sources = [("database", 1000), ("api", 500), ("cache", 200)];

    for source in sources {
        let handle = tokio::spawn(async move { fetch_data_async(source.0, source.1).await });
        handles.push(handle);
    }

    // Output appears all together at approx the time of the longest delay.
    for handle in handles {
        println!("{}", handle.await.unwrap());
    }

    println!("All async operations completed!");
}

pub async fn demonstrate_blocking() {
    println!("Starting blocking operations...");

    let mut handles = vec![];
    let sources = [("database", 1000), ("api", 500), ("cache", 200)];

    for source in sources {
        handles.push(tokio::spawn(async move {
            let data =
                tokio::task::spawn_blocking(move || fetch_data_sync(source.0, source.1)).await;
            data
        }));
    }

    // Output appears as the timers complete.
    while !handles.is_empty() {
        let (result, _index, remaining) = select_all(handles).await;
        println!("{}", result.unwrap().unwrap());
        handles = remaining;
    }

    println!("All blocking operations completed!");
}
