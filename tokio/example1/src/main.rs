use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

#[tokio::main]
async fn main() {
    // 创建一个通道
    let (tx, mut rx) = mpsc::channel(100);

    // 创建一个原子变量
    let local_data = AtomicI32::new(0);

    // 启动一个任务
    let task = tokio::spawn(async move {
        loop {
            tokio::select! {
                // 接收从通道发过来的数据
                Some(data) = rx.recv() => {
                    // 更新原子变量
                    local_data.store(data, Ordering::SeqCst);
                    println!("Recv data: {}", data);
                }
                // 每隔五分钟显示原子变量的值
                _ = time::sleep(Duration::from_secs(3)) => {
                    // 读取原子变量的值
                    let value = local_data.load(Ordering::SeqCst);
                    println!("Local data: {}", value);
                }
            }
        }
    });

    // 在主线程中发送数据到通道
    // 这里只是示例，您可以根据实际情况发送不同的数据
    for i in 1..=1000 {
        tx.send(i).await.unwrap();
        time::sleep(Duration::from_secs(1)).await;
    }

    task.await.unwrap();
}
