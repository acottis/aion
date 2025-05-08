use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

mod character;
mod data;
mod engine;
mod entity;
mod message;
mod session;
mod state;

use crossbeam_channel::{Receiver, Sender};
use krypt::game::{gen_xor_key, gen_xor_seed};
use message::{client as c, server as s, ClientMessages};
use session::Account;

use self::state::State;

const TICK_RATE: f32 = 144.;

trait Serialise {
    fn serialise(&self, buf: &mut [u8]) -> usize;
}

trait Deserialise {
    fn deserialise(buf: &[u8]) -> Self
    where
        Self: Sized;
}

/// Copy a buffer into a another buffer
/// ```
/// let mut len = 0;
/// let mut buf = [0u8; 100];
/// let data = [1,3,3,7];
///
/// copy_bytes!(len, buf, data);
/// ```
#[macro_export]
macro_rules! copy_bytes {
    ($ptr:expr, $buf:expr, $value:expr) => {
        let buffer_length = $value.len();
        $buf[$ptr..$ptr + buffer_length].copy_from_slice(&$value);
        $ptr += buffer_length;
    };
}

/// Serialise any value that implements .to_le_bytes() onto the given buffer
/// ```
/// let mut len = 0;
/// let mut buf = [0u8; 100];
/// let id: 32 = 13371337;
///
/// to_le_bytes!(len, buf, id);
/// ```
#[macro_export]
macro_rules! to_le_bytes {
    ($ptr:expr, $buf:expr, $value:expr) => {
        let size = core::mem::size_of_val(&$value);
        $buf[$ptr..$ptr + size].copy_from_slice(&$value.to_le_bytes());
        $ptr += size;
    };
}

/// Deserialise any value that implements .from_le_bytes() onto the given buffer
/// ```
/// let mut len = 0;
/// let mut buf = [0u8; 100];
///
/// let id = consume_le_bytes!(len, buf, u16);
/// ```
#[macro_export]
macro_rules! consume_le_bytes {
    ($ptr:expr, $buf:expr, $ty:ty) => {{
        let size = core::mem::size_of::<$ty>();
        let ret =
            <$ty>::from_le_bytes($buf[$ptr..$ptr + size].try_into().unwrap());
        $ptr += size;
        ret
    }};
}

#[derive(Debug, Clone)]
pub struct ServerUpdate {
    message: s::Message,
}
impl ServerUpdate {
    pub fn new(message: s::Message) -> Self {
        Self { message }
    }
    pub fn message(&self) -> &s::Message {
        &self.message
    }
}

#[derive(Clone)]
pub struct ClientUpdate {
    message: c::Message,
    client_id: u16,
    character_id: u32,
}

impl ClientUpdate {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn character_id(&self) -> u32 {
        self.character_id
    }
    pub fn message(&self) -> &c::Message {
        &self.message
    }
    pub fn new(client_id: u16, character_id: u32, message: c::Message) -> Self {
        Self {
            message,
            client_id,
            character_id,
        }
    }
}

/// Long lived buffers that store ServerUpdates during calculation before they
/// are sent to the players
struct Messages {
    /// Only send to the requesting client
    pub direct: HashMap<u16, Vec<ServerUpdate>>,
    /// Only send to other clients
    pub others: HashMap<u16, Vec<ServerUpdate>>,
    /// Send to all clients
    pub broadcast: Vec<ServerUpdate>,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            direct: HashMap::with_capacity(1000),
            others: HashMap::with_capacity(1000),
            broadcast: Vec::with_capacity(1000),
        }
    }
}

/// Propagates events from one clients to all relevent clients
/// Takes a turn per game tick for the server actions
pub fn game_update(
    clients: Arc<Mutex<HashMap<u16, Sender<ServerUpdate>>>>,
    server_rx: Receiver<ClientUpdate>,
) {
    let mut disconnected_clients = Vec::with_capacity(100);
    let mut state = State::new();
    let mut messages = Messages::new();

    let mut now = Instant::now();
    loop {
        // Only update on tick rate
        if now.elapsed() < Duration::from_secs_f32(1. / TICK_RATE) {
            std::thread::yield_now();
            continue;
        }
        now = Instant::now();

        // Respond to client messages
        for update in server_rx.try_iter() {
            // Does any additional server logic for an individual update
            state.respond(&update, &mut messages);
        }

        // Server turn
        state.update(&mut messages);

        // After calculating all updates we lock the clients list
        // to send them the updates
        let mut clients = clients.lock().unwrap();

        // Broadcast messages from a client to other clients
        for message in messages.broadcast.drain(..) {
            println!("[BROADCAST]: {:X?}", message.message());
            for (id, client) in clients.iter() {
                if client.send(message.clone()).is_err() {
                    disconnected_clients.push(*id);
                    break;
                }
            }
        }

        // Send Direct Messages
        for (client_id, messages) in messages.direct.drain() {
            let client = clients.get(&client_id).unwrap();
            for message in messages {
                println!("[DIRECT:{:X}]: {:X?}", client_id, message.message());
                if client.send(message).is_err() {
                    disconnected_clients.push(client_id);
                    break;
                }
            }
        }

        // Send others to only
        for (client_id, messages) in messages.others.drain() {
            for (id, client) in clients.iter() {
                if id == &client_id {
                    println!("[OTHERS:{:X}]: {:X?}", client_id, messages);
                    continue;
                }
                for message in messages.clone() {
                    if client.send(message).is_err() {
                        disconnected_clients.push(client_id);
                        break;
                    }
                }
            }
        }

        // Remove clients we cant send to
        for client in disconnected_clients.drain(..) {
            // We do not care if its already removed, this is just for saftey
            _ = clients.remove(&client);
        }
    }
}

pub struct Keys {
    client: [u8; 8],
    server: [u8; 8],
}

impl Keys {
    fn new(initial_key: [u8; 8]) -> Self {
        Self {
            client: initial_key,
            server: initial_key,
        }
    }
}

pub struct Connection {
    stream: TcpStream,
    keys: Keys,
    account: Account,
    /// The game server tx to clien rx
    rx: Receiver<ServerUpdate>,
    /// The client tx to the game servers rx
    tx: Sender<ClientUpdate>,
}

impl Connection {
    pub fn new(
        mut stream: TcpStream,
        client_id: u16,
        rx: Receiver<ServerUpdate>,
        tx: Sender<ClientUpdate>,
    ) -> Self {
        // Send this accross the network
        let seed = gen_xor_seed();

        // calculated key we keep in memory
        let key = gen_xor_key(seed);

        // S_KEY Packet
        let mut buf = [0u8; 11];
        let len = message::make_key_packet(&mut buf, seed);
        stream.write(&buf[..len]).unwrap();

        Self {
            stream,
            keys: Keys::new(key.to_le_bytes()),
            account: Account::new(client_id),
            rx,
            tx,
        }
    }

    pub fn handle(mut self) {
        loop {
            let mut recv_buffer = [0u8; 1024 * 64];
            let messages = match self.stream.read(&mut recv_buffer) {
                // TCP len 0 means close
                Ok(0) => return,
                Ok(len) => ClientMessages::deserialise(
                    &mut recv_buffer[..len],
                    &mut self.keys.client,
                ),
                Err(_) => ClientMessages::empty(),
            };

            let mut send_buffer = [0u8; 1024 * 64];
            let len = messages.handle(
                &mut send_buffer,
                &mut self.keys.server,
                &mut self.account,
                &self.rx,
                &self.tx,
            );
            if len != 0 {
                self.stream.write(&send_buffer[..len]).unwrap();
            }
        }
    }
}
