use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::{arg, command, value_parser};

use self::req::fetcher::Fetcher;

mod parser;
mod req;

#[tokio::main]
async fn main() -> Result<()> {
    let arg_config = parse_cli_arg();

    let file_path = if arg_config.output == "" {
        None
    } else {
        Some(arg_config.output.as_str())
    };

    match Fetcher::download(arg_config.url, file_path).await {
        Err(err) => {
            println!("Err: {:?}", err);
        }
        Ok(_) => {}
    }

    Ok(())
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

    arg_config
}
