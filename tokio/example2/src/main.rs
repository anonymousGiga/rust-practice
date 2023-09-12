use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    let local_data = Arc::new(AtomicI32::new(0));

    let local_data_clone = local_data.clone();
    let receiver_task = tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            local_data_clone.store(data, Ordering::SeqCst);
            println!("Recv data: {}", data);
        }
    });

    let sender_task = tokio::spawn(async move {
        for i in 1..=500 {
            tx.send(i).await.unwrap();
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    let main_task = tokio::spawn(async move {
        // thread::sleep(Duration::from_secs(2));
        loop {
            thread::sleep(Duration::from_secs(3));
            let value = local_data.load(Ordering::SeqCst);
            println!("Local data: {}", value);
        }
    });

    tokio::select! {
        _ = receiver_task => {},
        _ = sender_task => {},
        _ = main_task => {},
    };
}
