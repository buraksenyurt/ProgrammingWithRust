use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
pub async fn main() {
    do_with_tokio().await;
}

pub async fn do_with_tokio() {
    let (transmitter, mut receiver) = mpsc::channel(10);

    for i in 1..=5 {
        let tx_clone = transmitter.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            tx_clone
                .send(format!("Task {} completed", i))
                .await
                .unwrap();
        });
    }

    drop(transmitter);

    println!("Waiting for all tasks...");

    /*
       Standart mpsc örneğinden farklı olarak burada ana thread bloklanmadan
       döngünün asenkron olarak çalıştırılması sağlanır.
    */
    tokio::spawn(async {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Main task is working...Counting {}", i);
        }
    });

    while let Some(message) = receiver.recv().await {
        println!("{}", message);
    }

    println!("All tasks completed!");
}
