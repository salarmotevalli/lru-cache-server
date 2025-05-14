mod command;
mod lru;

use std::{env, ops::Index, sync::Arc};

use command::Command;
use lru::{Lru, LruConfig};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

struct Server {
    cmd: Command,
}

impl Server {
    pub fn new(cmd: Command) -> Self {
        Self { cmd }
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

struct Flags {
    capacity: usize,
    port: u16,
}

fn parse_args() -> Result<Flags, String> {
    let args: Vec<String> = env::args().collect();
    let mut capacity = 100; // default value
    let mut port = 6379;     // default value

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -c".to_string());
                }
                capacity = args[i + 1].parse().map_err(|_| "Invalid concurrency value".to_string())?;
                i += 1; // skip the next argument since we've processed it
            },
            "-p" => {
                if i + 1 >= args.len() {
                    return Err("Missing value for -p".to_string());
                }
                port = args[i + 1].parse().map_err(|_| "Invalid port value".to_string())?;
                i += 1; // skip the next argument
            },
            _ => return Err(format!("Unknown flag: {}", args[i])),
        }
        i += 1;
    }

    Ok(Flags { capacity, port })
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let flags = parse_args()?;
    let lru = Lru::new(LruConfig { capacity: flags.capacity });
    let cmd = Command::new(lru);
    let server = Arc::new(Server::new(cmd));

    let listener = TcpListener::bind(format!("127.0.0.1:{}", flags.port)).await.unwrap();
    println!("server running on {} with {} capacity", flags.port, flags.capacity);

    while let Ok(stream) = listener.accept().await {
        let cloned_server = server.clone();
        tokio::spawn(async move {
            cloned_server.handle(stream.0).await;
        });
    }

    Ok(())
}
