use std::ops::Add;
use std::sync::{Arc, Mutex};
use futures::future::join_all;

async fn add(counter: Arc<Mutex<u64>>) {
    let mut _counter = counter.lock().unwrap();
    *_counter = _counter.add(1);
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u64));

    let tasks = (0..10_000)
        .map(async |_| {
            tokio::spawn(add(Arc::clone(&counter)))
        });
    let _ = join_all(tasks).await;
    println!("Counter: {}", *counter.lock().unwrap());
}

