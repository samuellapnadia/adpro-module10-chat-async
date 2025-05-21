use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) = 
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    println!("Ella's Computer - From server: Welcome to chat! Type a message");

    loop {
        tokio::select! {
            user_input = stdin.next_line() => {
                if let Ok(Some(line)) = user_input {
                    ws_stream.send(Message::text(line)).await?;
                } else {
                    break;
                }
            }

            server_msg = ws_stream.next() => {
                if let Some(Ok(msg)) = server_msg {
                    if let Some(text) = msg.as_text() {
                        println!("Ella's Computer - From server: {text}");
                    }
                } else {
                    break;
                }
            }
        }
    }

    Ok(())
}
