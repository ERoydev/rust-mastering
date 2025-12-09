use std::{env::consts::DLL_EXTENSION, io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::env;
use std::{thread, time::Duration};
// constants
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    // read arguments
    let delay = env::args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();
    println!("Passed Delay: {}", delay);

    // starting 
    println!("Sirocco Echo Server is starting {}", SIROCCO_SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();

    // start
    println!("Sirocco listening {}", SIROCCO_SERVER_ADDRESS);

    for stream in listener.incoming() { // blocks and waits for new incoming TCP connections, on each connection yields a new stream
        let stream = stream.unwrap();
        println!("\n===== Connection established! =====");

        handle_connection(stream, delay);
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[0..len]);
    println!("Received: {}", message);
    
    // delay
    thread::sleep(Duration::from_millis(delay));

    // write the buffer
    let _ = stream.write_all(message.as_bytes());
    println!("Sent: {}", message);
}