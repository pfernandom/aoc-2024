use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);

    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = tx1.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    tokio::spawn(async move {
        for i in 10..20 {
            if let Err(_) = tx.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }
}
