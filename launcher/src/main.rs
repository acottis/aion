use std::{
    io::{stdin, stdout, BufWriter, ErrorKind, Write},
    process::Command,
    sync::{Arc, Mutex},
};

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
    // TODO: JWT is too big for gameforge auth token, have to switch
    let jwt: String = (0..20).map(|_| "A").collect();
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
                todo!("Add an auth token source");
                //let jwt = auth::login();
                //*state.lock().unwrap() = jwt;
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
