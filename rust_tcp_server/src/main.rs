use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast::{channel, Sender};
use tokio::time::{Duration, interval};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, _rx) = channel::<String>(10); // Broadcast channel
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    // Spawn a task to send periodic broadcast messages
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3)); // Broadcast every 3 seconds
        loop {
            interval.tick().await;
            let message = "Periodic broadcast message!\r\n".to_string();
            if tx_clone.send(message).is_err() {
                break; // Stop if there are no more receivers
            }
        }
    });

    println!("Listening on port 8080");

    // Accept incoming connections
    while let Ok((stream, _addr)) = listener.accept().await {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, tx_clone).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_client(mut stream: tokio::net::TcpStream, tx: Sender<String>) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1024];
    let mut receiver = tx.subscribe();

    loop {
        tokio::select! {
            // Read from the client
            result = stream.read(&mut buf) => {
                let n = result?;
                if n == 0 {
                    break; // Client disconnected
                }
                // Handle client message (if needed)
                let message = String::from_utf8_lossy(&buf[..n]);
                println!("Received from client: {}", message);
            }

            // Receive broadcast messages
            result = receiver.recv() => {
                let message = result?;
                stream.write_all(message.as_bytes()).await?;
            }
        }
    }

    Ok(())
}
