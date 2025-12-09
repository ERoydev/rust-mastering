use std::io::prelude::*;
use std::net::TcpStream;

/*
I want to use that to connect to the echo server instead of using `netcat` via TCP
*/

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    // connection

    println!("Conneting to {}", ECHO_SERVER_ADDRESS);
    
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        // Connected
        let stream_addr = stream.local_addr().unwrap(); // I take the socketAddress from the stream 
        println!("Connected to echo server {}:{}", stream_addr.ip(), stream_addr.port());

        // Write a hello world message
        let message = "Hello World!";
        let _ = stream.write(message.as_bytes());

        // When write to a stream (TCPStream), the data may be temporarily stored in an internal buffer for efficiency.
        // Calling .flush() forces the buffer to be written out immediately, ensuring the server receives your message right away.
        let _ = stream.flush(); 
        println!("sent: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);

    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }

}