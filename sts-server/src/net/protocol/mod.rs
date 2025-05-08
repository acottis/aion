use serde::Serialize;
use std::{fmt::Display, str::from_utf8};

mod api;
mod body;

use crate::error::{Error, Result};

use body::{Body, Reply};

use self::api::{Api, Presence};

use super::Session;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RequestHeader {
    method: Method,
    api: Api,
    protocol: Protocol,
    length: usize,
    sequence: Option<u16>,
}

#[allow(dead_code)]
impl RequestHeader {
    pub fn new() -> Self {
        Self {
            method: Method::Post,
            api: Api::Presence(Presence::UserInfo),
            protocol: Protocol::Sts1_0,
            length: 0,
            sequence: None,
        }
    }

    pub fn len(&self) -> &usize {
        &self.length
    }

    pub fn serialise(&self, buffer: &mut [u8]) -> usize {
        let sequence = if let Some(seq) = self.sequence {
            format!("s:{seq}R\r\n")
        } else {
            "".into()
        };

        let header = format!(
            "{method} {api} {protocol}\r\n\
            l:{len}\r\n\
            {sequence}\r\n",
            method = self.method,
            protocol = self.protocol,
            api = self.api,
            len = self.length,
        );
        buffer[..header.len()].copy_from_slice(header.as_bytes());

        header.len()
    }

    pub fn deserialise(raw: &str) -> Result<Self> {
        let mut headers = raw.split("\r\n");
        let mut first_line = headers
            .next()
            .ok_or(Error::InvalidHeaders)?
            .split_whitespace();
        let method: Method = first_line.next().try_into()?;
        let api: Api = first_line.next().try_into()?;
        let protocol: Protocol = first_line.next().try_into()?;

        // Parse any additional lines
        let mut length = 0;
        let mut sequence = None;
        while let Some(header) = headers.next() {
            let mut opcode = header.split(":");
            match opcode.next() {
                Some("l") => {
                    length = opcode
                        .next()
                        .ok_or(Error::ParsingHeaderLength)?
                        .parse::<usize>()
                        .map_err(Error::HeaderLength)?
                }
                Some("s") => {
                    sequence = Some(
                        opcode
                            .next()
                            .ok_or(Error::ParsingSequence)?
                            .parse::<u16>()
                            .map_err(Error::HeaderSequence)?,
                    )
                }
                _ => {}
            }
        }

        Ok(Self {
            method,
            api,
            protocol,
            length,
            sequence,
        })
    }
}

impl Default for RequestHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct ResponseHeader {
    protocol: Protocol,
    length: usize,
    sequence: Option<u16>,
}

impl ResponseHeader {
    fn new(sequence: Option<u16>) -> Self {
        Self {
            protocol: Protocol::Sts1_0,
            length: 0,
            sequence,
        }
    }

    fn serialise(&self, buffer: &mut [u8]) -> usize {
        let sequence = if let Some(seq) = self.sequence {
            format!("s:{seq}R\r\n")
        } else {
            "".into()
        };

        let header = format!(
            "{protocol} 200 OK\r\n\
            l:{len}\r\n\
            {sequence}\r\n",
            protocol = self.protocol,
            len = self.length
        );
        buffer[..header.len()].copy_from_slice(header.as_bytes());

        header.len()
    }
}

#[derive(Debug)]
enum Method {
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Post => write!(f, "POST"),
        }
    }
}

impl TryFrom<Option<&str>> for Method {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some("POST") => Ok(Self::Post),
            Some(value) => Err(Error::MethodUnimplemented(value.to_owned())),
            None => Err(Error::MethodMissing),
        }
    }
}

#[derive(Debug)]
enum Protocol {
    Sts1_0,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sts1_0 => write!(f, "STS/1.0"),
        }
    }
}

impl TryFrom<Option<&str>> for Protocol {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some("STS/1.0") => Ok(Self::Sts1_0),
            Some(value) => Err(Error::ProtocolUnimplemented(value.to_owned())),
            None => Err(Error::ProtocolMissing),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    header: ResponseHeader,
    body: Reply,
}

impl Response {
    fn serialise(&mut self, buffer: &mut [u8]) -> usize {
        let mut xml = String::with_capacity(buffer.len());
        let mut ser = quick_xml::se::Serializer::new(&mut xml);
        ser.expand_empty_elements(false);
        self.body.serialize(ser).unwrap();

        self.header.length = xml.len();
        let len = self.header.serialise(buffer);

        buffer[len..len + xml.len()].copy_from_slice(&xml.as_bytes());
        len + xml.len()
    }
}

#[derive(Debug)]
pub struct Request {
    header: RequestHeader,
    body: Body,
}

#[allow(dead_code)]
impl Request {
    pub fn deserialise(buffer: &[u8]) -> Result<(Self, usize)> {
        let text = from_utf8(buffer).map_err(Error::PacketNotValidUtf8)?;
        println!("DEBUG: Recieved\r\n{text}");
        let mut split_text = text.split("\r\n\r\n");

        let raw_headers = split_text.next().ok_or(Error::EmptyPacket)?;
        let header = RequestHeader::deserialise(raw_headers)?;

        let raw_body = split_text.next().ok_or(Error::MissingPacketBody)?;
        let raw_body = &raw_body
            .get(..*header.len())
            .ok_or(Error::PacketLenOverflow)?;
        let body = if raw_body.len() != 0 {
            Body::deserialise(raw_body)?
        } else {
            Body::Empty
        };

        let msg_len = header.len().checked_add(raw_headers.len() + 4);

        let message = Request { header, body };
        Ok((message, msg_len.ok_or(Error::PacketLenOverflow)?))
    }

    pub fn serialise(&mut self, buffer: &mut [u8]) -> usize {
        let xml = quick_xml::se::to_string(&self.body).unwrap();
        self.header.length = xml.len();

        let len = self.header.serialise(buffer);

        buffer[len..len + xml.len()].copy_from_slice(&xml.as_bytes());
        len + xml.len()
    }

    pub fn handle(&self, session: &mut Session, buffer: &mut [u8]) -> usize {
        let seq = self.header.sequence;
        match self.header.api.handle(session, self, seq) {
            Some(mut reply) => reply.serialise(buffer),
            None => 0,
        }
    }
}
