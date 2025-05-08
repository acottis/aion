use std::{
    io::{stdin, stdout, BufWriter, ErrorKind, Write},
    process::Command,
    sync::{Arc, Mutex},
};

mod auth;
mod gameforge;

//const GAME_PATH: &str =
//    r#"C:\Program Files\GameforgeGames\aionclassic\bin64\aion.bin"#;
const GAME_PATH: &str =
    r#"C:\tmp\aionclassic28-20240701\aionclassic\bin64\aion.bin"#;
const AUTH_SERVER: &str = "127.0.0.1";
const STS_SERVER: &str = "127.0.0.1";

const PROVIDER: Option<AuthProvider> = Some(AuthProvider::Gameforge);

enum AuthProvider {
    Gameforge,
}

fn launch_game() {
    println!("INFO: Logging in to game server: {STS_SERVER}");
    println!("INFO: Logging in to auth server: {AUTH_SERVER}");

    let mut command = Command::new(GAME_PATH);

    command
        // Prevents lookup to STS server Api/Goods
        .arg("-dnpshop")
        // Locale
        .arg("-lang:ENG");
    // Dont launch website after closing??
    //.arg("-noweb");
    //.arg("-nowebshop")
    //.arg("-dwsm")
    //.arg("-pwd16")
    //.arg("-ingamewebshop");
    //.arg("-npsa")
    //.arg("-probability")
    //.arg("-nospamcheck")
    //.arg("-st")
    //.arg("-dpetition")
    //.arg("-recserveridx:1")
    //.arg("-old_charInstance")
    //.arg("-litelauncher")
    //.arg("-noprivacycheck")
    //.arg("-ncg")
    //.arg("-noauthgg")
    //.arg("-ls")
    //.arg("-charnamemenu")
    //.arg("-ingameshop")
    //.arg(r#"-DEVMODE "g_con_disable_console 0"#)
    //.arg(r#"-DEVMODE "g_chatlog 1"#)

    match PROVIDER {
        Some(AuthProvider::Gameforge) => {
            command
                .env("_TNT_SESSION_ID", "")
                .env("_TNT_CLIENT_APPLICATION_ID", "")
                .arg("-usehqserver")
                .arg(format!("-hqip:{}", AUTH_SERVER))
                .arg("-hqport:13001")
                .arg(format!("-np:{}", STS_SERVER))
                .arg("-loginex");
        }
        None => {
            command.arg("-ip:127.0.0.1").arg("-port:6600");
        }
    }

    let process = command
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn();

    match process {
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            println!("ERROR: Game not found at path: {GAME_PATH}")
        }
        Err(e) => panic!("{e}"),
        _ => (),
    }
}

fn main() {
    let jwt = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6ImltaTBZMnowZFlLeEJ0dEFxS19UdDVoWUJUayJ9.eyJhdWQiOiJiYTc1YTM1OS1mODdjLTRhZjktODY3Ni0wNzM4NmZiZjU3ODUiLCJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vMTMyY2E1NTYtZmU0OS00ZDYwLWFlZDctMWY4MzY2Zjc5M2VhL3YyLjAiLCJpYXQiOjE3NDEyMjA2NjksIm5iZiI6MTc0MTIyMDY2OSwiZXhwIjoxNzQxMjI0NTY5LCJhaW8iOiJBV1FBbS84WkFBQUFNMVFQbE1adUNOTTE5U0lkQVBtOEFMUmhZS1hlbkdlZlFhajNaOXREMXV0bldJMmErdGg4R3J3d1dNRVVRR3IxTGdlNFRUSkxrMW1kRHJHS3dpL0xDcDg2SGl2WENMMjYxaU9VWE5QaDR5dnZ0NnBMODdmR2U1Rm8vOFo2TVI3cCIsIm5hbWUiOiJBZGFtIENvdHRpcyIsIm9pZCI6ImFiN2IzMDMxLWZhMjgtNGVlOC1hZmQxLTRlMDNlNDM4YTliNiIsInByZWZlcnJlZF91c2VybmFtZSI6ImFkYW1AYXNoZG93bi5zY290IiwicmgiOiIxLkFhOEFWcVVzRTBuLVlFMnUxeC1EWnZlVDZsbWpkYnA4LVBsS2huWUhPRy1fVjRWS0FVQ3ZBQS4iLCJzaWQiOiI1MGUzZmYzYS1hNjljLTQyYzYtOTMxYS1jYTE2Mzg3YTFlMTQiLCJzdWIiOiJxSXFNQmJsU3RsTDRQc2VnQno3S1JYcWJOSU0xZzFoRXNYSmUzWVpVanBrIiwidGlkIjoiMTMyY2E1NTYtZmU0OS00ZDYwLWFlZDctMWY4MzY2Zjc5M2VhIiwidXRpIjoiQVQ1Q1V1UVpDa3FraURpYUpLSThBQSIsInZlciI6IjIuMCJ9.mGX2bJ4H36kMw7f6OF8lc8SI_s8rozh02Hk3cbbxv83i1rTcjifB5GCacAtN2PxlTHfaWFZ7AQPwHBSjvtFYR6t2bMRjdtQPLxNTYbjOUFcs_3FTTJb2i7iohMHWVavxTv3hbvuEFpqqkJXvPPZJAJTfUPNOrmCqcIeNYCJhu7V3sRU_X2Pq6oa7aCLma9d3NDqMOaMyh3QxqQmbHSOd8oGh32_ElsBk54x2KllgJ1Xgf14FYPbe2kS4ZZvQBv_33jh7w5Kn1qiBHGf1S99x8BO_ty0jotVWEyl5jJDKeRRZ3B7jh8uRKW59odnDRlSlZBvxSP04XkAcBJnISmXYqA");
    let jwt: String = (0..700).map(|_| "A").collect();
    println!("{}", jwt.len());
    let state = Arc::new(Mutex::new(jwt));

    let gameforge_state = Arc::clone(&state);
    std::thread::spawn(|| gameforge::listen(gameforge_state));

    let mut w = BufWriter::new(stdout());
    let r = stdin();

    loop {
        writeln!(w, "1. Log in").unwrap();
        writeln!(w, "2. Launch game").unwrap();
        writeln!(w, "3. Exit").unwrap();
        write!(w, "Enter choice: ").unwrap();
        w.flush().unwrap();

        let mut input = String::new();
        r.read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                let jwt = auth::login();
                *state.lock().unwrap() = jwt;
            }
            "2" => {
                launch_game();
            }
            "3" => {
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
