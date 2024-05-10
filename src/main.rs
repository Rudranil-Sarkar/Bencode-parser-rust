use std::{env, fs, io};

use bencode::decode;

fn read_torrent_file(filepath: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(filepath)
}

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_bytes = read_torrent_file(&filepath).unwrap();

    let x = decode::decode_bencode_element(file_bytes).unwrap();

    println!("{}", x)
}