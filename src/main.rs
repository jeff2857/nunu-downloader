use std::path::PathBuf;

use clap::{arg, command, value_parser};

use self::parser::url_parser;
use self::req::fetcher::Fetcher;

mod parser;
mod req;

#[tokio::main]
async fn main() {
    let arg_config = parse_cli_arg();

    if let Err(url_invalid) = url_parser::UrlParser::parse_index_page(&arg_config.url) {
        println!("{}", url_invalid);
    }

    if let Err(err) = Fetcher::download(arg_config.url).await {
        println!("Err: {:?}", err);
    }
}

#[derive(Default, Debug)]
struct ArgConfig {
    url: String,
    output: String,
}

fn parse_cli_arg() -> ArgConfig {
    let matches = command!()
        .arg(arg!(
            <url> "Url of the novel's index page"
        ))
        .arg(
            arg!(
                -o --output <file> "Set output file name"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let mut arg_config = ArgConfig::default();

    if let Some(url) = matches.get_one::<String>("url") {
        let mut url = url.to_string();
        if url.ends_with("/") {
            url.push_str("index.html");
        }
        if !url.ends_with("index.html") {
            url.push_str("/index.html");
        }
        arg_config.url = url;
    }

    if let Some(file_path) = matches.get_one::<PathBuf>("output") {
        arg_config.output = file_path.to_str().unwrap().to_string();
    }

    arg_config
}
