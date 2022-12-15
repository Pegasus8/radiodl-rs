use std::io::{Read, Write};
use std::fs::File;
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();

    let url = "https://azul-1.nty.uy/";

    let mut response = client.get(url).send().unwrap();

    let mut file = File::create("radio.mp3").unwrap();

    let mut buffer = [0; 1024];

    while let Ok(len) = response.read(&mut buffer) {
        if len == 0 {
            break;
        }

        file.write_all(&buffer[..len]).unwrap();
    }
}
