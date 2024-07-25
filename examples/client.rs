use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;

    let user_text = "AI Chatbot: Hello, my name is ".to_string();
    let message = Message { text: user_text };
    let message = serde_json::to_vec(&message)?;

    socket.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await?;

    let response: SerdeResult<Message> = serde_json::from_slice(&buf[..n]);
    if let Ok(response) = response {
        println!("Completed text: {}", response.text);
    }

    Ok(())
}

