mod command;

use std::sync::Arc;

use command::Command;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

struct Server {
    cmd: Command,
}

impl Server {
    pub fn new() -> Self {
        Self {
            cmd: Command::new(),
        }
    }

    pub async fn handle(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = match stream.read(&mut buffer).await {
                Ok(0) => return, // client disconnected
                Ok(n) => n,
                Err(_) => return,
            };

            // Imagine you've parsed the command already
            // For demo, we'll just always return +PONG\r\n
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);

            let response = self.cmd.parse(request).await; // RESP Simple String

            if stream.write_all(response.as_bytes()).await.is_err() {
                return; // client connection broken
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Arc::new(Server::new());

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Redis-like server running on 6379");

    while let Ok(stream) = listener.accept().await {
        let cloned_server = server.clone();
        tokio::spawn(async move {
            cloned_server.handle(stream.0).await;
        });
    }
}
