mod request;
mod response;

use crate::request::{parse_request, RequestType};
use crate::response::{Response, StatusCode};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use std::io;

async fn handle_connection(mut socket: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 4096];
    let size = socket.read(&mut buffer).await?;

    let request_str = String::from_utf8_lossy(&buffer).to_string();
    let request = parse_request(request_str);
    let response = match request.request_type {
        RequestType::Get => {
            match request.path.as_str() {
                "/" => Response {
                    code: StatusCode::Ok,
                    body: "<h1>Hello, World!</h1>".to_string(),
                },
                _ => Response {
                    code: StatusCode::NotFound,
                    body: "Not Found".to_string()
                },
            }
        },
        _ => panic!("Error: Unimplemented Request"),
    };

    socket.write_all(response.into_string().as_ref()).await?;
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
