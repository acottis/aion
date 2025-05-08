pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SetStreamTimeout(std::io::Error),
    PeerAddr(std::io::Error),
}
