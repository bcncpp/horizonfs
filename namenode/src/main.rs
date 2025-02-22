mod rpc;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the TCP listener to the desired address and port.
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        // Accept an incoming connection.
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        // Spawn a new task for each connection.
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            // Read data in a loop and echo it back to the client.
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        // Connection closed by the client.
                        println!("Connection closed from {}", addr);
                        return;
                    }
                    Ok(n) => {
                        // Echo back the received data.
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("Failed to write to socket; error = {:?}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket; error = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
