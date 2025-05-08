mod aion;

use std::path::Path;

use krypt::game;
use pcap::{Capture, Offline};

use crate::aion::{client_message_from_opcode, server_message_from_opcode};

const DATA_START: usize = 0x36;
const PORT_START: usize = 0x22;

const SEVER_PORT: u16 = 0x1E61;

fn main() {
    //run(r#"C:\Code\hack\aion25\login-to-char-select.pcapng"#);
    //run(r#"C:\Code\hack\aion25\opengame-loginworld.pcapng"#);
    // run(r#"C:\Code\hack\aion25\login-game-trace1.pcapng"#);

    // Newer
    run(r#"C:\Code\hack\aion25\fightmossbearlogout.pcapng"#);
    // run(r#"C:\Code\hack\aion25\newversion.pcapng"#);
    // run(r#"C:\Code\hack\aion25\massivevarious.pcapng"#);
    //run(r#"C:\Code\hack\aion25\emoteslogout.pcapng"#);
    //run(r#"C:\Code\hack\aion25\putusershard.pcapng"#);
    //run(r#"C:\Code\hack\aion25\foo.pcapng"#);
}

fn run(path: impl AsRef<Path>) {
    let mut cap = Capture::from_file(path).unwrap();

    // Only show port 7777 tcp with the flags psh ack
    cap.filter(
        "tcp port 7777 && (tcp[13] == 0x10 || tcp[13] == 0x18) && (ip[2:2] != 40)",
        true,
    )
    .unwrap();

    let mut client_key = parse_key(&mut cap);
    let mut server_key = client_key.clone();
    println!("KEY: {:X?}", client_key);

    let log = std::env::var("LOG");
    let mut server_payload: Vec<u8> = Vec::with_capacity(64 * 1024);
    let mut client_payload: Vec<u8> = Vec::with_capacity(64 * 1024);
    while let Ok(packet) = cap.next_packet() {
        let port: [u8; 2] =
            packet.data[PORT_START..PORT_START + 2].try_into().unwrap();
        let port = u16::from_be_bytes(port);

        if port == SEVER_PORT {
            server_payload.extend_from_slice(&packet.data[DATA_START..]);
            if packet.data[14 + 20 + 13] == 0x10 {
                continue;
            }
            let messages = parse_packet_payload(&server_payload);
            server_payload.clear();
            println!();
            for mut message in messages {
                game::decrypt(&mut server_key, &mut message);
                let opcode = u16::from_le_bytes([message[0], message[1]]);
                let d_opcode = game::decrypt_server_opcode(opcode);
                let name = server_message_from_opcode(d_opcode);
                println!("S {name} {d_opcode:04X} ({opcode:04X})");
                //                if d_opcode == 0x37 {
                //                    let m = aion::SMoveNew::deserialise(&message[5..]);
                //                    println!("{m:02X?}");
                //                }
                //                println!(
                //                    "{}",
                //                    std::string::String::from_utf8_lossy(&message[5..])
                //                );
                if log == Ok("VERBOSE".into()) {
                    println!("{:02X?}", &message[5..]);
                }
            }
        } else {
            client_payload.extend_from_slice(&packet.data[DATA_START..]);
            if packet.data[14 + 20 + 13] == 0x10 {
                continue;
            }
            let messages = parse_packet_payload(&client_payload);
            client_payload.clear();
            println!();
            for mut message in messages {
                game::decrypt(&mut client_key, &mut message);
                let opcode = u16::from_le_bytes([message[0], message[1]]);
                let d_opcode = game::decrypt_client_opcode(opcode);
                let name = client_message_from_opcode(d_opcode);
                println!("C {name} {d_opcode:04X} ({opcode:04X})");
                //                if d_opcode == 0x30 {
                //                    let m = aion::CMoveNew::deserialise(&message[5..]);
                //                    println!("{m:02X?}");
                //                }
                //                println!(
                //                    "{}",
                //                    std::string::String::from_utf8_lossy(&message[5..])
                //                );
                if log == Ok("VERBOSE".into()) {
                    println!("{:02X?}", &message[5..]);
                }
            }
        }
    }
}

fn parse_packet_payload(mut data: &[u8]) -> Vec<Vec<u8>> {
    let mut messages = Vec::new();
    loop {
        if data.len() < 2 {
            break;
        }
        let len = u16::from_le_bytes([data[0], data[1]]) as usize;
        if len <= data.len() {
            messages.push(data[2..len].to_vec());
            data = &data[len..];
        } else {
            println!("{}, {}", data.len(), len);
            todo!();
        }
    }
    messages
}

fn parse_key(cap: &mut Capture<Offline>) -> [u8; 8] {
    let s_key = cap.next_packet().unwrap();
    let key: [u8; 4] = s_key.data[s_key.data.len() - 4..].try_into().unwrap();
    let key = u32::from_le_bytes(key);

    game::gen_xor_key(key).to_le_bytes()
}
