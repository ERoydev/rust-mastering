use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};


// Here i use both of sync sirocco and the tokio karin

// constants
const KARIN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // starting 
    println!("Karin Echo Server is starting {}", KARIN_SERVER_ADDRESS);

    // bind
    let listener = TcpListener::bind(KARIN_SERVER_ADDRESS).await.unwrap();

    // start
    println!("Karin listening {}", KARIN_SERVER_ADDRESS);

    loop {
        let (tcp_stream, _) = listener.accept().await.unwrap();

        // When i do that handle connection we are still on the same thread
        // handle_connection(tcp_stream).await;

        // The idea is on every new connection i need to spawn a new socket connection(thread) where this connection is going to handle the tasks
        tokio::spawn(async move {
            handle_connection(tcp_stream).await;
        });

    }
}

async fn handle_connection(mut stream: TcpStream) {
    // read the buffer
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[0..len]);
    println!("Received: {}", message);
    
    // call sirocco
    let sirocco_message = call_sirroco(message.to_owned().to_string()).await;
    let output = format!("sirocco says {}", sirocco_message);

    // write the buffer
    let _ = stream.write_all(output.as_bytes()).await;
    println!("Karin sent: {}", output);
}


const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

async fn call_sirroco(message: String) -> String {
    // connection

    println!("Conneting to sirocco: {}", SIROCCO_SERVER_ADDRESS);
    
    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS).await {
        // Connected
        let stream_addr = stream.local_addr().unwrap(); // I take the socketAddress from the stream 
        println!("Connected to sirocco {}:{}", stream_addr.ip(), stream_addr.port());

        // Write a hello world message
        let _ = stream.write_all(message.as_bytes()).await.unwrap();
        println!("sent to sirocco: {}", message);

        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[0..len]);
        println!("received from sirocco: {}", message);

        return message.to_owned().to_string();
    } else {
        println!("failed to connect to sirocco {}", ECHO_SERVER_ADDRESS);
        String::from("Failed").to_owned()
    }

}