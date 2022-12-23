use clap::{Parser, ValueEnum};
use message_types::{PartialTag, ServerResponse, UIRequest};
use serde_json;
use table_print::Table;
use termsize;
use tungstenite::{connect, Message};
use url::Url;

mod message_types;

#[derive(ValueEnum, Debug, Clone)]
enum SousaCommands {
    Play,
    Pause,
    Search,
    SwitchTo,
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
    #[arg(index = 2, requires("field"))]
    search_arg: Option<String>,

    /// The field to search for when running `search`
    #[arg(
        long,
        required_if_eq_any([("action", "search"), ("action", "SwitchTo")]),
        value_parser(["title", "artist", "album"])
    )]
    field: Option<String>,
    // Add flag for "search filepaths too"
}

fn main() {
    let cli = CliArgs::parse();

    let (mut socket, _resp) = connect(
        Url::parse(format!("ws://{}:{}", cli.hostname.unwrap(), cli.port.unwrap()).as_str())
            .unwrap(),
    )
    .expect("Couldn't connect to url");

    let message_string = match cli.action.unwrap() {
        SousaCommands::Play => serde_json::to_string(&UIRequest::Play).unwrap(),
        SousaCommands::Pause => serde_json::to_string(&UIRequest::Pause).unwrap(),
        SousaCommands::Search => {
            let request = match parse_to_partialtag(cli.field.unwrap(), cli.search_arg.unwrap()) {
                Ok(tag) => UIRequest::Search(tag),
                Err(_) => panic!(
                    "Unknown Search type! Expected values are 'title', 'artist', and 'album'"
                ),
            };
            serde_json::to_string(&request).unwrap()
        }
        SousaCommands::SwitchTo => {
            let request = match parse_to_partialtag(cli.field.unwrap(), cli.search_arg.unwrap()) {
                Ok(tag) => UIRequest::SwitchTo(tag),
                Err(_) => panic!("Unknown type!"),
            };
            serde_json::to_string(&request).unwrap()
        }
    };

    socket.write_message(Message::Text(message_string)).unwrap();
    let msg = socket.read_message().expect("Error reading message");
    let resp: message_types::ServerResponse =
        serde_json::from_str(msg.into_text().unwrap().as_str()).unwrap();
    println!("\n{}\n", resp.message);

    if resp.search_results.len() > 0 {
        resp.pretty_print();
    }
    //println!("recieved: {:?}\n{:?}", resp.message, resp.search_results);
    socket.close(None).unwrap();
}

fn parse_to_partialtag(field: String, value: String) -> Result<PartialTag, ()> {
    match field.to_lowercase().as_str() {
        "title" => Ok(PartialTag {
            title: Some(value),
            ..PartialTag::default()
        }),
        "artist" => Ok(PartialTag {
            artist: Some(value),
            ..PartialTag::default()
        }),
        "album" => Ok(PartialTag {
            album: Some(value),
            ..PartialTag::default()
        }),
        _ => Err(()),
    }
}

impl ServerResponse {
    fn pretty_print(self: &Self) -> () {
        let mut table = Table::new(vec![
            "Title".to_string(),
            "Artist".to_string(),
            "Album".to_string(),
        ]);
        for i in self.search_results.iter() {
            table.insert_row(vec![i.title.clone(), i.artist.clone(), i.album.clone()]);
        }
        println!(
            "{}",
            table
                .get_pretty(termsize::get().unwrap().cols as usize)
                .unwrap()
        );
    }
}
