use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    // Membuat "radio penerima" untuk klien ini dari saluran utama
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // PINTU 1: Mendengarkan pesan yang dikirim oleh Klien ini ke Server
            incoming = ws_stream.next() => {
                let msg = match incoming {
                    Some(Ok(msg)) => msg,
                    Some(Err(e)) => return Err(e.into()),
                    None => return Ok(()), // Klien terputus
                };

                if let Some(text) = msg.as_text() {
                    println!("From client {addr} {text:?}");
                    
                    // Format Eksperimen 2.3: Tambahkan IP dan Port pengirim!
                    let formatted_msg = format!("{}: {}", addr, text);
                    
                    // Siarkan pesan ke SEMUA klien lain yang terhubung
                    let _ = bcast_tx.send(formatted_msg);
                }
            }
            // PINTU 2: Mendengarkan pesan dari Klien LAIN yang disiarkan oleh Server
            msg = bcast_rx.recv() => {
                let msg = msg?;
                // Teruskan pesan siaran tersebut ke Klien ini
                ws_stream.send(Message::text(msg)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Kita menggunakan 'if let' sebagai ganti dari tanda '?'
            // Jika koneksi websocket sukses (Ok), maka jalankan handle_connection
            if let Ok((_req, ws_stream)) = ServerBuilder::new().accept(socket).await {
                let _ = handle_connection(addr, ws_stream, bcast_tx).await;
            } else {
                println!("Gagal memproses koneksi websocket dari {addr:?}");
            }
        });
    }
}