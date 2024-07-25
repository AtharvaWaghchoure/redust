use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;
    let _ = stream.write_all(b"set name atharva");
    Ok(())
}
