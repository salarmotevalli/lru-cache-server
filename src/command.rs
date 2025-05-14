use std::{borrow::Cow, sync::Arc};

use crate::lru::{value::Value, Lru};
use tokio::sync::Mutex;

pub struct Command {
    lru: Arc<Mutex<Lru>>,
}

impl Command {
    pub fn new(lru: Lru) -> Self {
        Self {
            lru: Arc::new(Mutex::new(lru)),
        }
    }

    pub async fn parse(&self, cmd: Cow<'_, str>) -> String {
        let lines: Vec<&str> = cmd.lines().collect();

        // Minimal RESP parsing: handle only Arrays and Bulk Strings
        if lines.is_empty() || !lines[0].starts_with('*') {
            return "-ERR invalid command\r\n".into();
        }

        // Extract command and arguments
        let mut parts = Vec::new();
        let mut i = 1;
        while i < lines.len() {
            if lines[i].starts_with('$') {
                if i + 1 >= lines.len() {
                    return "-ERR malformed bulk string\r\n".into();
                }
                parts.push(lines[i + 1]);
                i += 2;
            } else {
                i += 1;
            }
        }

        if parts.is_empty() {
            return "-ERR empty command\r\n".into();
        }

        match parts[0].to_uppercase().as_str() {
            "SET" => {
                if parts.len() != 3 {
                    return "-ERR wrong number of arguments for 'SET'\r\n".into();
                }

                let key = parts[1].to_string();
                let value = parts[2];

                // Allow only strings and ints
                if value.parse::<i64>().is_ok() || !value.contains('\0') {
                    let mut store = self.lru.lock().await;
                    store.set(key, Value::Str(value.to_string())).await;
                    println!("{:?}", store);
                    "+OK\r\n".into()
                } else {
                    "-ERR value must be string or int\r\n".into()
                }
            }

            "GET" => {
                if parts.len() != 2 {
                    return "-ERR wrong number of arguments for 'GET'\r\n".into();
                }

                let key = parts[1];
                let mut store = self.lru.lock().await;
                match store.get(key.to_string()).await {
                    Some(val) => {
                        let v = val.to_string();
                        
                        println!("{:?}", store);
                        format!("${}\r\n{}\r\n", v.len(), v)
                    }
                    None => "$-1\r\n".into(),
                }
            }
            "PING" => "+PONG\r\n".into(),

            _ => "-ERR unknown command\r\n".into(),
        }
    }
}
