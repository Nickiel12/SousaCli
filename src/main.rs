use clap::{Parser, ValueEnum};
use message_types::UIRequest;
use serde_json;
use tungstenite::{connect, Message};
use url::Url;

mod message_types;

#[derive(ValueEnum, Debug, Clone)]
enum SousaCommands {
    Play,
    Pause,
    Search,
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
    #[arg(index = 1, value_enum)]
    action: Option<SousaCommands>,

    /// The string to search for when paired with a "Search" action
    #[arg(index = 2, required_if_eq("action", "search"))]
    search_arg: Option<String>,

    /// The field to search for when running `search`
    #[arg(
        short,
        long,
        required_if_eq("action", "search"),
        value_parser(["title", "artist", "album", "album_artist"])
    )]
    search_field: Option<String>,
}

fn main() {
    let cli = CliArgs::parse();

    let (mut socket, resp) = connect(
        Url::parse(format!("ws://{}:{}", cli.hostname.unwrap(), cli.port.unwrap()).as_str())
            .unwrap(),
    )
    .expect("Couldn't connect to url");

    let message_string = match cli.action.unwrap() {
        SousaCommands::Play => serde_json::to_string(&UIRequest::Play).unwrap(),
        SousaCommands::Pause => serde_json::to_string(&UIRequest::Pause).unwrap(),
        SousaCommands::Search => String::new(),
    };

    socket.write_message(Message::Text(message_string)).unwrap();
    //let msg = socket.read_message().expect("Error reading message");
    //println!("Received: {}", msg);
    socket.close(None).unwrap();
}
