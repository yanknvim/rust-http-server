use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use std::io;

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 4096];
    let r = socket.read(&mut buffer).await?;

    socket.write_all(&buffer).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> { 
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        handle_connection(socket).await?;
    }
}
