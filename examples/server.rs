use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use gpt::completer::{Completer, GptCompleter};
use serde::{Serialize, Deserialize};
use serde_json::Result as SerdeResult;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => {
                    let msg: SerdeResult<Message> = serde_json::from_slice(&buf[..n]);
                    if let Ok(msg) = msg {
                        let gpt_completer = GptCompleter;
                        match gpt_completer.complete(&msg.text).await {
                            Ok(completed_text) => {
                                let response = Message { text: completed_text };
                                let response = serde_json::to_vec(&response).unwrap();
                                socket.write_all(&response).await.unwrap();
                            }
                            Err(e) => eprintln!("Error generating completion: {}", e),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                }
            }
        });
    }
}

