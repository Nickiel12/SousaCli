use clap::{Parser, ValueEnum};
use message_types::{itemtag_to_partial, PartialTag, ServerResponse, UIRequest};
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
    StatusUpdate,
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

    /// The value used to search/switch tracks
    #[arg(
        index = 2,
        requires("field"),
        required_if_eq_any([("action", "search"), ("action", "SwitchTo")])
    )]
    search_arg: Option<String>,

    #[arg(long, required_if_eq("action", "SwitchTo"))]
    choice_index: Option<usize>,

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
        SousaCommands::StatusUpdate => {
            let request = UIRequest::GetStatus;
            serde_json::to_string(&request).unwrap()
        }
    };

    socket.write_message(Message::Text(message_string)).unwrap();
    let server_message = socket.read_message().expect("Error reading message");

    let server_response: message_types::ServerResponse =
        serde_json::from_str(server_message.into_text().unwrap().as_str()).unwrap();

    println!("{}", server_response.message.clone());
    if server_response
        .message
        .starts_with("Multiple results found")
    {
        if cli.choice_index.is_some() {
            let index = cli.choice_index.unwrap();
            if index > server_response.search_results.len() {
                println!("That index was larger than the list of options.\nTry running the command again without the `--choice-index` flag");
            } else {
                println!("Sending second server request");
                let return_choice = server_response.search_results.get(index).unwrap();
                socket
                    .write_message(
                        serde_json::to_string(&UIRequest::SwitchTo(itemtag_to_partial(
                            return_choice,
                        )))
                        .unwrap()
                        .into(),
                    )
                    .unwrap();
            }
        } else {
            choose_switch_to(server_response);
        }
    } else {
        println!("\n{}\n", server_response.message);

        if server_response.search_results.len() > 0 {
            server_response.pretty_print();
        }
    }
    //println!("recieved: {:?}\n{:?}", resp.message, resp.search_results);
    println!("Closing Socket");
    socket.close(None).unwrap();
}

fn choose_switch_to(msg: ServerResponse) {
    println!("\n\n{}\n\n", msg.message);

    let mut table = Table::new(vec![
        "Index".to_string(),
        "Title".to_string(),
        "Artist".to_string(),
        "Album".to_string(),
    ]);

    let mut count: usize = 0;
    for i in msg.search_results.iter() {
        table.insert_row(vec![
            count.to_string(),
            i.title.clone(),
            i.artist.clone(),
            i.album.clone(),
        ]);
        count += 1;
    }
    println!(
        "{}",
        table
            .get_pretty(termsize::get().unwrap().cols as usize)
            .unwrap()
    );

    println!(
        "Please enter the index of the desired song as an arguement. e.g.: `--choice-index=1`"
    );
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
