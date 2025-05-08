mod error;
mod net;

use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::time::Duration;

use crate::error::Error;
use crate::net::Session;

fn main() {
    println!("INFO: Starting STS Server");
    let listener = TcpListener::bind("0.0.0.0:6600").unwrap();
    println!("INFO: Listening on {}", listener.local_addr().unwrap());

    while let Ok((stream, _)) = listener.accept() {
        spawn(|| handle(stream).unwrap());
    }
}

fn handle(stream: TcpStream) -> error::Result<()> {
    println!(
        "INFO: new session {}",
        stream.peer_addr().map_err(Error::PeerAddr)?
    );

    stream
        .set_read_timeout(Some(Duration::from_secs(60)))
        .map_err(Error::SetStreamTimeout)?;
    stream
        .set_write_timeout(Some(Duration::from_secs(60)))
        .map_err(Error::SetStreamTimeout)?;

    Session::new().handle(stream)
}
