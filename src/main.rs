use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(mut socket: TcpStream) {
    println!("New connection");
    let mut buf = vec![0; 1024];
    let len = socket.read(&mut buf).await.unwrap();

    println!("Received {} {}", String::from_utf8_lossy(&buf[..len]), len);

    socket.write_all(b"Hello from server").await.unwrap();
}