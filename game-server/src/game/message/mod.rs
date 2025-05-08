use crossbeam_channel::{Receiver, Sender};
use krypt::game::encrypt;

pub mod client;
pub mod server;

use crate::game::{session::Account, Serialise};

use super::{ClientUpdate, ServerUpdate};

pub struct ClientMessages {
    messages: Vec<client::Message>,
}

impl ClientMessages {
    pub fn empty() -> Self {
        Self {
            messages: Vec::with_capacity(0),
        }
    }
    pub fn deserialise(mut buffer: &mut [u8], key: &mut [u8; 8]) -> Self {
        let mut messages = Vec::new();

        loop {
            if buffer.len() < 2 {
                break;
            }

            let len = u16::from_le_bytes([buffer[0], buffer[1]]) as usize;
            if len > buffer.len() || len < 2 {
                panic!("Invalid Packet Len");
            }

            if let Some(message) =
                client::Message::deserialise(&mut buffer[2..len], key)
            {
                println!("C: {message:02X?}");
                messages.push(message);
            }

            buffer = &mut buffer[len..];
        }
        Self { messages }
    }

    pub fn handle(
        self,
        mut buf: &mut [u8],
        key: &mut [u8; 8],
        account: &mut Account,
        rx: &Receiver<ServerUpdate>,
        tx: &Sender<ClientUpdate>,
    ) -> usize {
        let mut total_len = 0;

        // Thread local messages
        for message in self.messages {
            let replies = message.handle(&tx, account);

            for reply in replies {
                println!("[LOCAL]: {reply:02X?}");

                let len = reply.serialise(buf);
                encrypt(key, &mut buf[2..len]);
                buf = &mut buf[len..];
                total_len += len;
            }
        }

        // Messages from another thread
        for update in rx.try_iter() {
            let message = update.message();

            let len = message.serialise(buf);
            encrypt(key, &mut buf[2..len]);
            buf = &mut buf[len..];
            total_len += len;
        }
        total_len
    }
}

/// Make a S_KEY packet to be the first message from server to client
pub fn make_key_packet(buf: &mut [u8; 11], key: u32) -> usize {
    let key = server::Message::Key(server::Key::new(key));
    key.serialise(buf)
}
