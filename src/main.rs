use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.events";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to the agent network");

    let (mut write, mut read) = ws_stream.split();
    let msg = Message::Text("Willy wonka server".into());

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read");
        println!("Received a message : {}", message);
    }

    print!("Sending message : {}", msg);

    write.send(msg).await.expect("Failed to send the message");

    if let Some(message) = read.next().await {
        let message = message.expect("Failed to read");
        println!("Received a message : {}", message);
    }
}
