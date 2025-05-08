mod protocol;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

use crate::error::{Error, Result};
use krypt::sts::{Krypt, Rsa};
use protocol::Request;

const RECV_BUFFER_LEN: usize = 1024 * 64;
const SEND_BUFFER_LEN: usize = 1024 * 64;

pub struct Session {
    pub krypt: Krypt,
    pub authn_token: String,
}

impl Session {
    pub fn new() -> Self {
        let rsa_bits = 0x800;
        Self {
            krypt: Krypt {
                rsa: Rsa::new(rsa_bits),
                rc4: None,
                // TODO: Make random
                server_rand: *b"foofoofoofoofoofoofoofoofoofoofo",
                client_rand: [0u8; 0x20],
            },
            authn_token: String::new(),
        }
    }
    pub fn handle(mut self, mut stream: TcpStream) -> Result<()> {
        let mut requests: Vec<Request> = Vec::with_capacity(20);

        // Unencrypted loop
        loop {
            let mut recv_buffer = [0u8; RECV_BUFFER_LEN];
            let len = stream
                .read(&mut recv_buffer)
                .map_err(Error::ReadingFromSocket)?;

            self.parse_packets(&recv_buffer[..len], &mut requests)?;

            for request in requests.drain(..) {
                let mut send_buffer = [0u8; SEND_BUFFER_LEN];
                let len = request.handle(&mut self, &mut send_buffer);
                if len != 0 {
                    println!("DEBUG: Sending message");
                    println!("{}", from_utf8(&send_buffer[..len]).unwrap());
                    stream
                        .write(&send_buffer[..len])
                        .map_err(Error::WritingToSocket)?;
                }
            }

            // If we have enabled RC4 move on to the encrypted loop
            if self.krypt.rc4.is_some() {
                break;
            }
        }

        // Encrypted loop
        let mut rc4 = self.krypt.rc4.take().unwrap();
        loop {
            let mut recv_buf = [0u8; RECV_BUFFER_LEN];
            let len = stream
                .read(&mut recv_buf)
                .map_err(Error::ReadingFromSocket)?;
            rc4.decrypt(&mut recv_buf[..len]);

            self.parse_packets(&recv_buf[..len], &mut requests)?;

            for request in requests.drain(..) {
                let mut send_buf = [0u8; SEND_BUFFER_LEN];
                let len = request.handle(&mut self, &mut send_buf);
                if len != 0 {
                    println!("DEBUG: Sending message");
                    println!("{}", from_utf8(&send_buf[..len]).unwrap());
                    rc4.encrypt(&mut send_buf[..len]);
                    stream
                        .write(&send_buf[..len])
                        .map_err(Error::WritingToSocket)?;
                }
            }
        }
    }

    /// There can be one more more message in a packet
    fn parse_packets(
        &mut self,
        mut buffer: &[u8],
        requests: &mut Vec<Request>,
    ) -> Result<()> {
        loop {
            let (message, len) = Request::deserialise(buffer)?;
            requests.push(message);

            let remaining_bytes = buffer.len().checked_sub(len);
            match remaining_bytes {
                Some(0) => return Ok(()),
                Some(_) => buffer = &buffer[len..],
                None => return Err(Error::PacketLenOverflow),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialise_connect() {
        const PAYLOAD: &str = "POST /Sts/Connect STS/1.0\r\n\
            l:222\r\n\r\n\
            <Connect> <ConnType>400</ConnType> \
            <AppIndex>1</AppIndex> <Build>1001</Build> <ProductType>1020\
            </ProductType> <Address>69.69.69.69</Address> <Process>28025\
            </Process> <Epoch>7205</Epoch> <Program>3092</Program> </Connect>\
            \r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        session
            .parse_packets(PAYLOAD.as_bytes(), &mut requests)
            .unwrap();
    }

    #[test]
    fn deserialise_connect_len_too_short() {
        const PAYLOAD: &str = "POST /Sts/Connect STS/1.0\r\n\
            l:221\r\n\r\n\
            <Connect> <ConnType>400</ConnType> \
            <AppIndex>1</AppIndex> <Build>1001</Build> <ProductType>1020\
            </ProductType> <Address>69.69.69.69</Address> <Process>28025\
            </Process> <Epoch>7205</Epoch> <Program>3092</Program> </Connect>\
            \r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        assert!(matches!(
            session.parse_packets(PAYLOAD.as_bytes(), &mut requests),
            Err(_)
        ));
    }

    #[test]
    fn deserialise_login_token_start() {
        const PAYLOAD:&str = "POST /Auth/LoginTokenStart STS/1.0\r\n\
          s:1\r\n\
          l:90\r\n\r\n\
          <Request><ClientRand>5e+U2ifE4U0N+pFGggUkd94xrvvrsKzzFej1j2c7L5Y=</ClientRand></Request>\r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        session
            .parse_packets(PAYLOAD.as_bytes(), &mut requests)
            .unwrap();
    }

    #[test]
    fn deserialise_login_token_start_len_too_long() {
        const PAYLOAD:&str = "POST /Auth/LoginTokenStart STS/1.0\r\n\
          s:1\r\n\
          l:91\r\n\r\n\
          <Request><ClientRand>5e+U2ifE4U0N+pFGggUkd94xrvvrsKzzFej1j2c7L5Y=</ClientRand></Request>\r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        assert!(matches!(
            session.parse_packets(PAYLOAD.as_bytes(), &mut requests),
            Err(Error::PacketLenOverflow)
        ));
    }

    #[test]
    fn deserialise_concat() {
        const PAYLOAD: &str = "POST /Sts/Connect STS/1.0\r\n\
            l:222\r\n\r\n\
            <Connect> <ConnType>400</ConnType> \
            <AppIndex>1</AppIndex> <Build>1001</Build> <ProductType>1020\
            </ProductType> <Address>69.69.69.69</Address> <Process>28025\
            </Process> <Epoch>7205</Epoch> <Program>3092</Program> </Connect>\
            \r\n\
            POST /Auth/LoginTokenStart STS/1.0\r\n\
            s:1\r\n\
            l:90\r\n\r\n\
            <Request><ClientRand>5e+U2ifE4U0N+pFGggUkd94xrvvrsKzzFej1j2c7L5Y=</ClientRand></Request>\r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        session
            .parse_packets(PAYLOAD.as_bytes(), &mut requests)
            .unwrap();
    }

    #[test]
    fn deserialise_token_data_start() {
        const PAYLOAD:&str = "POST /Auth/TokenKeyData STS/1.0\r\ns:2\r\nl:1175\r\n\r\n<Request><EncryptionAlgorithm>AES</EncryptionAlgorithm><EncryptionKey>aj3MBPPrx0eoH3dK9eVrqoXSFH6AlOUmGME26USwcrW1df+S8Swl2t0igbv56g+pJPcSxfnd845oFVrOPhpKssMS0LDNh39fY36uTXIeo7cGey0Sd0kQkbPs8NVpCrq5CZwBKeD/MbeXFygHElEzLN/it15GZ10RsU3eI1LV9EKIMm3XnD6eVk3/p0IRmM24GLJ5y9Z/1Kn45d70j4hP5OxJx2I4XpNok8nukJMhE5MrLusQKtLkrJtitz/18epNwZ/9haLk+4m7/LD5aizw/Gj4jtRDr/MZB4xluGeulq7HXS8GXq3s42shlNAVMATWb3QFyzlOmMpC1e3nDkN0Tg==</EncryptionKey><PremasterSecret>7n2o5J2TCN+VRHFL0YHvZ2IjbAgedgXuNx/3dTW40tGogQud0kuqnUD5W0awI+bz</PremasterSecret><AuthnToken>F0WKQ+C8vjSy8xejuBSlH2NBnH34vnYexSazoGgpVcD9BdjJd464OD3tTAIyJgre9Fl3uBcp85j4lVgHMe8sGwJ6f+AGzSfNN9xyzVFljzILrYCqufqKgrTNDNRWMM6/4iCH+7xKuwXrY5ImyxKeKat/X5Jd9oPkE2IBw2rX/ssWX400oAkJnlBQ1tc+V/CTy05VvNJVBTl7NZ1ePHKQIcgsILzolIWdPOUI5FZJudzTSS8r9ZrJgk0YRBnKQKQOuW5p/mep4v2bK5sisADmTY2K9nl/0Jf3WSdpuJb4lScqGd8SImTK9OgC1BCAkYyDz+TWgd/5/MnllPAcsxzx4gp1h2Xoy6g/0nfHuxtJjHBQ0OJRNFDaX7tadsaz7NEAg6qQm1wuU0p9U2roZlZ8ue5j73qOvNd3RU8V5xFlOvgjZ6h8WsLGnTaVBH5jlfS2tC4PGCXa0/vWU9/xZ4pN1qPDJEkM6fQTQK8653EDvLHxN8furAC2aKiPPd6uMYe3</AuthnToken><AuthProviderCode>gameforge</AuthProviderCode><AppId>5B4503FA-8521-4608-BB60-5CFAC87BD63A</AppId></Request>\r\n";

        let mut session = Session::new();
        let mut requests = Vec::new();
        session
            .parse_packets(PAYLOAD.as_bytes(), &mut requests)
            .unwrap();
    }
}
