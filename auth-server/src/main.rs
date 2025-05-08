mod error;

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread::spawn;

fn main() {
    println!("INFO: Starting Auth Server");
    let listener = TcpListener::bind("0.0.0.0:13001").unwrap();
    println!("INFO: Listening on {}", listener.local_addr().unwrap());

    while let Ok((stream, _)) = listener.accept() {
        spawn(|| handle(stream));
    }
}

#[derive(Clone, Copy)]
struct GameServer {
    id: u8,
    ip_addr: Ipv4Addr,
    online: bool,
    port: u16,
    /// 0x4 is normal, 0x5 is Instance Server??
    typ: u8,
    characters: u8,
}

impl GameServer {
    const LEN: usize = 63;

    fn serialise(&self) -> [u8; Self::LEN] {
        let mut bytes = [0u8; Self::LEN];

        // Not sure what these are
        bytes[0] = 0x01;

        if self.typ == 0x4 {
            bytes[55] = 0x1;
        }

        bytes[1] = self.typ;
        bytes[4] = self.online as u8;
        bytes[5] = self.id;
        bytes[9] = self.characters;

        let ip_addr = self.ip_addr.to_string();
        bytes[13..13 + ip_addr.len()].copy_from_slice(ip_addr.as_bytes());

        let port: [u8; 2] = self.port.to_le_bytes();
        bytes[53..53 + port.len()].copy_from_slice(&port);

        bytes
    }
}

struct GameServerList {
    last_logged_on: u8,
    game_servers: [Option<GameServer>; Self::MAX],
    len: usize,
}

impl GameServerList {
    const SEQUENCE_NUMBER: u8 = 0x02;
    const PREAMPLE_LEN: usize = 8;
    const SUFFIX_LEN: usize = 4;
    const MAX: usize = 20;

    fn new(last_logged_on: u8) -> Self {
        Self {
            last_logged_on,
            len: 0,
            game_servers: [None; Self::MAX],
        }
    }

    fn serialise(&self, buf: &mut [u8; 1024 * 64]) -> usize {
        let mut len = Self::PREAMPLE_LEN;
        let mut game_servers_len = 0;

        let mut game_servers = self.game_servers.iter();
        while let Some(Some(game_server)) = game_servers.next() {
            buf[len..len + GameServer::LEN]
                .copy_from_slice(&game_server.serialise());

            len += GameServer::LEN;
            game_servers_len += 1;
        }

        buf[len] = self.last_logged_on;
        len += Self::SUFFIX_LEN;

        buf[0] = len as u8;
        buf[2] = Self::SEQUENCE_NUMBER;
        buf[6] = game_servers_len as u8;

        len
    }

    fn insert(&mut self, game_server: GameServer) {
        if self.len < Self::MAX {
            self.game_servers[self.len] = Some(game_server);
            self.len += 1;
        } else {
            panic!("Too many Game Servers");
        }
    }
}

fn handle(mut stream: TcpStream) {
    println!("INFO: incoming {}", stream.peer_addr().unwrap());

    // 5B4503FA-8521-4608-BB60-5CFAC87BD63A7282405F-79E5-49C2-9872-D4F3CC961FF3aiongfc
    let packet_bytes: [u8; 92] = [
        0x5c, 0x00, 0x05, 0x00, 0x00, 0x00, 0x35, 0x42, 0x34, 0x35, 0x30, 0x33,
        0x46, 0x41, 0x2d, 0x38, 0x35, 0x32, 0x31, 0x2d, 0x34, 0x36, 0x30, 0x38,
        0x2d, 0x42, 0x42, 0x36, 0x30, 0x2d, 0x35, 0x43, 0x46, 0x41, 0x43, 0x38,
        0x37, 0x42, 0x44, 0x36, 0x33, 0x41, 0x00, 0x37, 0x32, 0x38, 0x32, 0x34,
        0x30, 0x35, 0x46, 0x2d, 0x37, 0x39, 0x45, 0x35, 0x2d, 0x34, 0x39, 0x43,
        0x32, 0x2d, 0x39, 0x38, 0x37, 0x32, 0x2d, 0x44, 0x34, 0x46, 0x33, 0x43,
        0x43, 0x39, 0x36, 0x31, 0x46, 0x46, 0x33, 0x00, 0x61, 0x69, 0x6f, 0x6e,
        0x67, 0x66, 0x63, 0x00, 0x14, 0x0c, 0x00, 0x00,
    ];

    stream.write(&packet_bytes).unwrap();

    let mut buf = [0u8; 512];
    let len = stream.read(&mut buf).unwrap();
    println!("INFO: {:?}", std::str::from_utf8(&buf[..len]));

    let packet_bytes: [u8; 10] =
        [0x0a, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    stream.write(&packet_bytes).unwrap();

    let mut game_server_list = GameServerList::new(0x15);
    game_server_list.insert(GameServer {
        id: 0x15,
        ip_addr: Ipv4Addr::new(192, 168, 1, 153),
        online: true,
        port: 7777,
        typ: 4,
        characters: 1,
    });
    game_server_list.insert(GameServer {
        id: 0x16,
        ip_addr: Ipv4Addr::new(79, 110, 83, 113),
        online: true,
        port: 7777,
        typ: 4,
        characters: 0,
    });
    game_server_list.insert(GameServer {
        id: 0x67,
        ip_addr: Ipv4Addr::new(79, 110, 83, 114),
        online: false,
        port: 7777,
        typ: 5,
        characters: 0,
    });
    let mut packet_bytes = [0u8; 1024 * 64];
    let len = game_server_list.serialise(&mut packet_bytes);
    stream.write(&packet_bytes[..len]).unwrap();

    let mut buf = [0u8; 512];
    let len = stream.read(&mut buf).unwrap();
    println!("INFO: {:?}", std::str::from_utf8(&buf[..len]));

    // Last four bytes is key??
    let packet_bytes: [u8; 22] = [
        0x16, 0x00, 0x04, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00, 0xb6, 0xca,
        0x62, 0x02, 0x00, 0x00, 0x00, 0x00, 0x02, 0x31, 0x0A, 0x01,
    ];

    stream.write(&packet_bytes).unwrap();

    let mut buf = [0u8; 512];
    let len = stream.read(&mut buf).unwrap();
    println!("INFO: {:?}", std::str::from_utf8(&buf[..len]));

    println!("INFO: closing tcp {}", stream.peer_addr().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_server() {
        let gs = GameServer {
            id: 0x15,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1),
            online: true,
            port: 7777,
            typ: 4,
            characters: 1,
        };

        let target = [
            0x01, 0x04, 0x00, 0x00, 0x01, 0x15, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, b'1', b'2', b'7', 0x2e, b'0', 0x2e, b'0', 0x2e, b'1',
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x1e,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(gs.serialise(), target);
    }

    #[test]
    fn test_game_servers_list_single() {
        let mut gsl = GameServerList::new(0x15);
        gsl.insert(GameServer {
            id: 0x15,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1),
            online: true,
            port: 7777,
            typ: 4,
            characters: 1,
        });

        let mut buf = [0u8; 1024 * 64];
        let len = gsl.serialise(&mut buf);

        let target = [
            75, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x04, 0x00,
            0x00, 0x01, 0x15, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, b'1',
            b'2', b'7', 0x2e, b'0', 0x2e, b'0', 0x2e, b'1', 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x1e, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x00,
        ];
        assert_eq!(buf[..len], target);
    }

    #[test]
    fn test_game_servers_list_multi() {
        let mut gsl = GameServerList::new(0x15);
        gsl.insert(GameServer {
            id: 0x15,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1),
            online: true,
            port: 7777,
            typ: 4,
            characters: 1,
        });
        gsl.insert(GameServer {
            id: 0x67,
            ip_addr: Ipv4Addr::new(127, 0, 0, 1),
            online: false,
            port: 7777,
            typ: 5,
            characters: 0,
        });

        let mut buf = [0u8; 1024 * 64];
        let len = gsl.serialise(&mut buf);

        let target: [u8; 138] = [
            138, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x04, 0x00,
            0x00, 0x01, 0x15, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, b'1',
            b'2', b'7', 0x2e, b'0', 0x2e, b'0', 0x2e, b'1', 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x1e, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x00, 0x00, 0x00, 0x67,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, b'1', b'2', b'7', 0x2e,
            b'0', 0x2e, b'0', 0x2e, b'1', 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x61, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x15, 0x00, 0x00, 0x00,
        ];
        assert_eq!(buf[..len], target);
    }
}
