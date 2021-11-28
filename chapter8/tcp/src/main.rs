use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com";
    let port = 80;
    let space = "\r\n";

    let mut conn = TcpStream::connect(format!("{}:{}", host, port))?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(space.as_bytes())?;

    conn.write_all(format!("HOST: {}", host).as_bytes())?;
    conn.write_all(format!("{}{}", space, space).as_bytes())?;

    std::io::copy(&mut conn, &mut std::io::stdout())?;

    Ok(())
}
