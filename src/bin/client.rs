use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    println!("Welcome to chat! Type a message");

    // Loop tanpa henti untuk menjaga aplikasi tetap hidup
    loop {
        tokio::select! {
            // PINTU 1: Mendengarkan pesan masuk dari Server
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("Ade's Computer - From server: {}", text);
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()), // Koneksi ditutup oleh server
                }
            }
            // PINTU 2: Mendengarkan ketikan dari Keyboard kamu
            res = stdin.next_line() => {
                match res {
                    Ok(Some(line)) => {
                        // Mengirim teks yang diketik ke Server
                        ws_stream.send(Message::text(line)).await?;
                    }
                    Ok(None) => return Ok(()), // Aplikasi ditutup (EOF)
                    Err(err) => panic!("Error reading from stdin: {err}"),
                }
            }
        }
    }
}