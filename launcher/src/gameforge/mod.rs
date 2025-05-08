use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

mod wyn;

const GAME_LANG: &str = "en-GB";
const ACCOUNT_NAME: &str = "";
const ACCOUNT_NUMERIC_ID: &str = "";
const PIPE_NAME: &str = "\\\\.\\pipe\\GameforgeClientJSONRPC\0";
const MAX_INSTANCES: u32 = 255;

#[derive(Deserialize, Debug)]
struct RpcRequest {
    id: usize,
    jsonrpc: String,
    method: String,
    params: RpcParams,
}

#[derive(Deserialize, Debug)]
struct RpcParams {
    #[serde(rename = "sessionId")]
    session_id: String,
}

#[derive(Serialize, Debug)]
struct RpcResponse {
    id: usize,
    jsonrpc: String,
    result: RpcResult,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum RpcResult {
    Running(bool),
    SessionId(String),
    AuthCode(String),
    Locale(String),
    AccountName(String),
    AccountNumericId(String),
}

fn handle_message(
    message: &RpcRequest,
    state: &Arc<Mutex<String>>,
) -> Result<RpcResponse, ()> {
    println!("[GCLIENT_MOCK] DEBUG: {message:#?}");
    match message.method.as_str() {
        "ClientLibrary.isClientRunning" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::Running(true),
        }),
        "ClientLibrary.initSession" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::SessionId(message.params.session_id.clone()),
        }),

        "ClientLibrary.queryGameDisplayLocale" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::Locale(GAME_LANG.into()),
        }),
        // This is how we prove to aion who we are
        "ClientLibrary.queryAuthorizationCode" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::AuthCode(state.lock().unwrap().clone()),
        }),
        "ClientLibrary.queryGameAccountName" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::AccountName(ACCOUNT_NAME.into()),
        }),
        "ClientLibrary.queryGameAccountNumericId" => Ok(RpcResponse {
            id: message.id,
            jsonrpc: message.jsonrpc.clone(),
            result: RpcResult::AccountNumericId(ACCOUNT_NUMERIC_ID.into()),
        }),
        method => {
            unimplemented!("{}", method);
        }
    }
}

pub fn listen(state: Arc<Mutex<String>>) {
    // Check if pipe exists
    if File::open(PIPE_NAME).is_ok() {
        panic!("[GCLIENT] The pipe {PIPE_NAME} is already open");
    };

    println!("[GCLIENT] INFO: Starting mock gclient named pipe server");
    println!("[GCLIENT] INFO: Creating pipe with name {PIPE_NAME}");
    loop {
        let pipe = wyn::Pipe::new(PIPE_NAME, MAX_INSTANCES).unwrap();

        println!("[GCLIENT] INFO: Pipe listening...");
        pipe.listen().unwrap();

        println!("[GCLIENT] INFO: message recieved");
        let mut buffer = [0u8; 1024 * 10];
        let len = pipe.read(&mut buffer).unwrap();
        let message =
            serde_json::from_slice::<RpcRequest>(&buffer[..len as usize]);

        if let Ok(message) = message {
            if let Ok(response) = handle_message(&message, &state) {
                println!("[GCLIENT] DEBUG: sending {response:#?}");
                let payload = serde_json::to_string(&response).unwrap();
                pipe.write(payload.as_bytes()).unwrap();
            }
        } else {
            println!("[GCLIENT] ERR: Not a valid RpcRequest");
            println!("{:?}", std::str::from_utf8(&buffer[..len as usize]));
        }
        pipe.close().unwrap();
    }
}
