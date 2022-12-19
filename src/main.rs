use clap::{Parser, ValueEnum};
use tungstenite::{connect, Message};
use url::Url;

const POSSIBLE_COMMANDS: &[&str] = &["play", "pause"];

#[derive(ValueEnum, Debug, Clone)]
enum SousaCommands {
    Play,
    Pause,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct CliArgs {
    /// The IP of the Sousa server. Defaults to 'localhost'
    #[arg(long, default_value = "localhost")]
    hostname: Option<String>,

    /// The Port of the Sousa server. Defaults to something
    #[arg(long, default_value = "9001")]
    port: Option<String>,

    /// The command to execute
    #[arg(short, long, value_enum)]
    action: SousaCommands,
}

fn main() {
    let cli = CliArgs::parse();

    let (mut socket, resp) = connect(
        Url::parse(format!("ws://{}:{}", cli.hostname.unwrap(), cli.port.unwrap()).as_str())
            .unwrap(),
    )
    .expect("Couldn't connect to url");

    println!("Connected to the server");
    println!("Response HTTP code: {}", resp.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in resp.headers() {
        println!("* {}", header);
    }

    socket
        .write_message(Message::Text("Hello WebSocket".into()))
        .unwrap();
    //let msg = socket.read_message().expect("Error reading message");
    //println!("Received: {}", msg);
    socket.close(None).unwrap();
}
