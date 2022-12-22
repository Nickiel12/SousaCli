use clap::{Parser, ValueEnum};
use message_types::{PartialTag, UIRequest};
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
        value_parser(["title", "artist", "album"])
    )]
    search_field: Option<String>,
    // Add flag for "search filepaths too"
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
        SousaCommands::Search => {
            let request: UIRequest = match cli.search_field.unwrap().to_lowercase().as_str() {
                "title" => UIRequest::Search(PartialTag {
                    title: cli.search_arg,
                    ..PartialTag::default()
                }),
                "artist" => UIRequest::Search(PartialTag {
                    artist: cli.search_arg,
                    ..PartialTag::default()
                }),
                "album" => UIRequest::Search(PartialTag {
                    album: cli.search_arg,
                    ..PartialTag::default()
                }),
                _ => panic!(
                    "Unknown search type! Expected values are 'title', 'artist', and 'album'"
                ),
            };
            serde_json::to_string(&request).unwrap()
        }
    };

    socket.write_message(Message::Text(message_string)).unwrap();
    let msg = socket.read_message().expect("Error reading message");
    let resp: message_types::ServerResponse =
        serde_json::from_str(msg.into_text().unwrap().as_str()).unwrap();
    println!("recieved: {:?}", resp);
    socket.close(None).unwrap();
}
