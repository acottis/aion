//! This server is responsible for the game protocol, this protocol starts once
//! the client has picked a server, the IP Address of the choson server is this
//! game server instance.
//!
//! # The protocol
//! - The first two bytes of every message are the length, a packet *CAN* contain
//! more than one message
//! - The length which includes its own bytes is follow by an encrypted payload.
//! This payload is some kind of XOR shift with a hardcoded starting key of
//! `nKO/WctQ0AVLbpzfBkS6NevDYT8ourG5CRlmdjyJ72aswx4EPq1UgZhFMXH?3iI9`
//!
//! 1. The client begins the protocol by connecting to a game server instance and
//! waiting for a message
//! 2. The server sends a packet where bytes 0-4 of the payload are static
//! `[0xc8,0x01,0x40,0x37,0xfe]` then 4 bytes which are a random key
//! 3. The server then responds with a payload about the client version, looks
//! like OS version info from `GetVersionExA` and `GetACP` for the ANSI format

mod error;
mod game;

use crossbeam_channel::Sender;
use game::{ClientUpdate, Connection, ServerUpdate};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Duration;

use error::Error;

fn main() {
    println!("INFO: Starting Game Server");
    let listener = TcpListener::bind("0.0.0.0:7777").unwrap();
    println!("INFO: Listening on {}", listener.local_addr().unwrap());

    let clients = Arc::new(Mutex::new(HashMap::new()));
    let (server_tx, server_rx) = crossbeam_channel::bounded(1000);

    let connections = clients.clone();
    spawn(move || game::game_update(connections, server_rx));

    while let Ok((stream, _)) = listener.accept() {
        let connections = clients.clone();
        let server_tx = server_tx.clone();
        spawn(|| handle(stream, connections, server_tx).unwrap());
    }
}

fn handle(
    stream: TcpStream,
    clients: Arc<Mutex<HashMap<u16, Sender<ServerUpdate>>>>,
    server_tx: Sender<ClientUpdate>,
) -> error::Result<()> {
    let peer_addr = stream.peer_addr().map_err(Error::PeerAddr)?;
    let client_id = peer_addr.port();
    println!("INFO: new session {peer_addr}");

    stream
        .set_read_timeout(Some(Duration::from_millis(1)))
        .map_err(Error::SetStreamTimeout)?;
    //    stream
    //        .set_write_timeout(Some(Duration::from_secs(60)))
    //        .map_err(Error::SetStreamTimeout)?;

    let (client_tx, client_rx) = crossbeam_channel::bounded(100);
    clients.lock().unwrap().insert(client_id, client_tx);
    Connection::new(stream, client_id, client_rx, server_tx).handle();

    clients.lock().unwrap().remove(&peer_addr.port());
    println!("INFO: Connection closed from {peer_addr}");

    Ok(())
}
