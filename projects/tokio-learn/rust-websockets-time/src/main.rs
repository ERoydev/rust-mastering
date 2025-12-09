use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/*

Repo: https://github.com/chrishayuk/rust-websockets-time/blob/main/src/main.rs
*/

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.org/";

    println!("Connecting to - {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to Agent Network");

    let (mut write, mut read) = ws_stream.split();

    let msg: Message = Message::Text("aloha at echo server".into());

    println!("Sending message: {}", msg);

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read the message");
        println!("Received a message: {}", message);
    }

    write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read the message");
        println!("Received a message: {}", message);
    }
}   
