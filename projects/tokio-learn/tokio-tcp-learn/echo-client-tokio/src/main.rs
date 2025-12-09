use tokio::{net::TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/*
I want to use that to connect to the echo server instead of using `netcat` via TCP
*/

const ECHO_SERVER_ADDRESS: &str = "localhost:8001";

#[tokio::main]
async fn main() {
    // connection

    println!("Conneting to {}", ECHO_SERVER_ADDRESS);
    
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        // Connected
        let stream_addr = stream.local_addr().unwrap(); // I take the socketAddress from the stream 
        println!("Connected to echo server {}:{}", stream_addr.ip(), stream_addr.port());

        // Write a hello world message
        let message = "Hello World!";
        // let _ = stream.write(message.as_bytes());
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[0..len]);
        println!("received: {}", message);

    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }


}