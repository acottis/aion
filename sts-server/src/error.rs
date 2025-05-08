pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SetStreamTimeout(std::io::Error),
    PacketNotValidUtf8(std::str::Utf8Error),
    InvalidPacketBody(quick_xml::DeError),
    PeerAddr(std::io::Error),
    MissingPacketBody,
    ReadingFromSocket(std::io::Error),
    WritingToSocket(std::io::Error),
    ApiNamespaceUnimplemented(String),
    ApiFunctionUnimplemented(String),
    MethodUnimplemented(String),
    ParsingHeaderLength,
    HeaderLength(std::num::ParseIntError),
    HeaderSequence(std::num::ParseIntError),
    ParsingSequence,
    EmptyPacket,
    InvalidHeaders,
    PacketLenOverflow,
    MethodMissing,
    ProtocolUnimplemented(String),
    ProtocolMissing,
    InvalidApi,
}
