use std::{
    io::{Read, Write},
    net::TcpStream,
    process::Command,
    str::from_utf8,
    sync::Arc,
};

use rustls::{pki_types::ServerName, ClientConfig, ClientConnection, Stream};
use rustls_platform_verifier::BuilderVerifierExt;

const TENANT_ID: &str = "YOUR_TENANT_ID";
const HOST: &str = "login.microsoftonline.com";
const CLIENT_ID: &str = "YOUR_CLIENT_ID";

#[derive(Debug, serde::Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u32,
    interval: u8,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
struct TokenResponse {
    token_type: String,
    scope: String,
    expires_in: u32,
    ext_expires_in: u32,
    access_token: String,
    id_token: String,
}

struct TlsClient {
    client_config: Arc<ClientConfig>,
}

impl TlsClient {
    fn new() -> Self {
        let client_config = Arc::new(
            ClientConfig::builder()
                .with_platform_verifier()
                .with_no_client_auth(),
        );
        Self { client_config }
    }

    fn request(&self, address: &str, request: &[u8], buf: &mut [u8]) -> usize {
        let server_name = ServerName::try_from(HOST).unwrap();
        let mut conn =
            ClientConnection::new(self.client_config.clone(), server_name)
                .unwrap();
        let mut stream = TcpStream::connect(address).unwrap();
        let mut tls_stream = Stream::new(&mut conn, &mut stream);

        tls_stream.write_all(request).unwrap();
        tls_stream.read(buf).unwrap()
    }
}

/// Returns a JWT
pub fn login() -> String {
    let client = TlsClient::new();
    let device_code = device_code(&client);

    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(&device_code.verification_uri)
        .output()
        .unwrap();
    println!("{}", device_code.message);

    poll_token(&client, &device_code).unwrap().id_token
}

fn poll_token(
    client: &TlsClient,
    device_code: &DeviceCodeResponse,
) -> Result<TokenResponse, &'static str> {
    let mut timeout = device_code.expires_in;
    let poll_interval = device_code.interval;

    let addr = format!("{HOST}:443");

    let path = format!("/{TENANT_ID}/oauth2/v2.0/token");
    let body = format!(
        "client_id={CLIENT_ID}\
        &device_code={}\
        &grant_type=urn:ietf:params:oauth:grant-type:device_code",
        device_code.device_code,
    );
    let body_len = body.len();
    let request = format!(
        "POST {path} HTTP/1.1\r\n\
        Host: {HOST}\r\n\
        Content-Length: {body_len}\r\n\
        Content-Type: application/x-www-form-urlencoded\r\n\
        \r\n\
        {body}",
    );

    let mut buf = [0u8; 8 * 1024];
    loop {
        if timeout <= 0 {
            return Err("Could not get auth token without expires_in limit");
        }

        std::thread::sleep(std::time::Duration::from_secs(
            poll_interval.into(),
        ));
        let len = client.request(&addr, request.as_bytes(), &mut buf);
        let parts = from_utf8(&buf[..len]).unwrap();
        let mut first_line_parts = parts.lines().next().unwrap().split(' ');
        let status = first_line_parts.nth(1).unwrap();
        if status == "200" {
            let (_, body) = parts.split_once("\r\n\r\n").unwrap();
            return Ok(serde_json::from_str::<TokenResponse>(body).unwrap());
        }

        timeout -= poll_interval as u32;
    }
}

fn device_code(client: &TlsClient) -> DeviceCodeResponse {
    let addr = format!("{HOST}:443");

    let path = format!("/{TENANT_ID}/oauth2/v2.0/devicecode");
    let body = format!("client_id={CLIENT_ID}&scope=openid");
    let body_len = body.len();
    let request = format!(
        "POST {path} HTTP/1.1\r\n\
        Host: {HOST}\r\n\
        Content-Length: {body_len}\r\n\
        Content-Type: application/x-www-form-urlencoded\r\n\
        \r\n\
        {body}",
    );

    let mut buf = [0u8; 8 * 1024];
    let len = client.request(&addr, request.as_bytes(), &mut buf);
    let parts = from_utf8(&buf[..len]).unwrap();
    let (_headers, body) = parts.split_once("\r\n\r\n").unwrap();

    serde_json::from_str::<DeviceCodeResponse>(body).unwrap()
}
