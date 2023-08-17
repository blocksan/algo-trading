use tokio_tungstenite::connect_async;
use url::Url;
use std::error::Error;
use futures::StreamExt;

async fn connect_websocket(server_url: &str) -> Result<(), Box<dyn Error>> {

    let (mut ws_stream, _) = connect_async(Url::parse(server_url).unwrap()).await.unwrap();

    println!("Connected to WebSocket server: {}", server_url);

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(message) => {
                if message.is_text() {
                    let text = message.to_text().unwrap();
                    println!("Received on {} tick: {}",server_url, text);
                    // Here, you can process the received CSV data or perform any other actions.
                }
            }
            Err(e) => {
                eprintln!("Error while receiving message: {:?}", e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
   println!("differen than main file");
   let server_urls = vec![
        "ws://localhost:5554", // Replace this with your WebSocket server URLs
        "ws://localhost:5555",
        "ws://localhost:5556",
    ];

    let tasks = server_urls
        .into_iter()
        .map(|server_url| {
            tokio::spawn(async move {
                if let Err(e) = connect_websocket(server_url).await {
                    eprintln!("Error connecting to {}: {}", server_url, e);
                }
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(tasks).await;
}
