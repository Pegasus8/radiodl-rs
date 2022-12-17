use clap::Parser;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = Client::new();

    let mut response = client.get(args.url).send()?;

    let mut file = File::create(args.output)?;

    let mut buffer = [0; 1024];

    while let Ok(len) = response.read(&mut buffer) {
        if len == 0 {
            break;
        }

        file.write_all(&buffer[..len])?;
    }

    Ok(())
}
