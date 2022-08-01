use std::path::PathBuf;

use clap::{arg, command, value_parser};

use self::parser::url_parser;

mod parser;

fn main() {
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
        arg_config.url = url.to_string();
    }

    if let Some(file_path) = matches.get_one::<PathBuf>("output") {
        arg_config.output = file_path.to_str().unwrap().to_string();
    }

    println!("argConfig: {:?}", arg_config);

    // validate url and file_path
    if let Err(url_invalid) = url_parser::UrlParser::parse_index_page(&arg_config.output) {
        println!("{}", url_invalid);
    }
    // parse url
}

#[derive(Default, Debug)]
struct ArgConfig {
    url: String,
    output: String,
}
