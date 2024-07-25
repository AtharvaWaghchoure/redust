use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let mut storage: HashMap<String, String> = HashMap::new();
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    let data = String::from_utf8_lossy(&buffer[..size]);
                    let mut data = data.split_whitespace();

                    if let Some(command) = data.next() {
                        match command {
                            "set" => {
                                if let (Some(key), Some(value)) = (data.next(), data.next()) {
                                    storage.insert(key.to_string(), value.to_string());
                                    println!("value: {:?}", storage.get_key_value(key));
                                }
                            }
                            "get" => {
                                if let Some(key) = data.next() {
                                    match storage.get(key) {
                                        Some(value) => {
                                            println!("{}:{}", key, value);
                                            let response = format!("{}: {}\n", key, value);
                                            let _ = stream.write_all(response.as_bytes());
                                        }
                                        None => {
                                            let _ = stream.write_all(b"Key not found");
                                        }
                                    }
                                }
                            }
                            _ => println!("too lazy to do the rest"),
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from stream: {}", e);
            }
        }
    }
}
