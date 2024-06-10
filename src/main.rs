mod request;
use crate::request::RequestType;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use std::io;

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 4096];
    let size = socket.read(&mut buffer).await?;

    let request_str = String::from_utf8_lossy(&buffer)
        .split("\r\n")
        .collect();
    let request = request::parse_request(request_str.head());;
    let response = match request.request_type {
        RequestType::Get => {},
        RequestType::Post => {},
    };

    socket.write(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    socket.write(b"<h1>Hello!</h1>").await?;
    socket.flush().await?;

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
