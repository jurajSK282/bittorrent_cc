use hex::encode;
use serde_json;
use std::{collections::BTreeMap, env};
use std::panic;
// Available if you need it!
// use serde_bencode


struct Torrent {
    announce: String,
    info: Info,
}

struct Info {
    name: String,
    // piece length is the number of bytes in each piece
    piece_length: usize,
    // pieces is a string whose length is a multiple of 20
    pieces: Vec<u8>,
    length: Option<i64>,
    files: Option<Vec<File>>,
}

struct File {
    length: usize,
    path: Vec<String>,
}

fn deserialize_info(info: serde_json::Value) -> Result<Info, Box<dyn std::error::Error>> {
    let name = info["name"].as_str().unwrap().to_string();
    let piece_length = info["piece length"].as_u64().unwrap() as usize;
    let pieces = info["pieces"].as_str().unwrap().as_bytes().to_vec();
    let length = info.get("length").map(|l| l.as_i64().unwrap());
    let files = info.get("files").map(|files| {
        files
            .as_array()
            .unwrap()
            .iter()
            .map(|file| {
                let length = file["length"].as_u64().unwrap() as usize;
                let path = file["path"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| p.as_str().unwrap().to_string())
                    .collect();
                File { length, path }
            })
            .collect()
    });
    Ok(Info {
        name,
        piece_length,
        pieces,
        length,
        files,
    })
}

fn deserialize_torrent(torrent: serde_json::Value) -> Result<Torrent, Box<dyn std::error::Error>> {
    let announce = torrent["announce"].as_str().unwrap().to_string();
    let info = deserialize_info(torrent["info"].clone())?;
    Ok(Torrent { announce, info })
}

fn read_torrent_file(file_path: &str) -> Result<Torrent, Box<dyn std::error::Error>> {
    let file = std::fs::read(file_path)?;
    let mut remainder: &[u8] = &[];
    let result = decode_bencoded_value(&file, &mut remainder);
        
    deserialize_torrent(result)
}

#[allow(dead_code)]
fn decode_bencoded_value<'a>(encoded_value: &'a [u8], remainder: &mut &'a[u8]) -> serde_json::Value {
    let (tag, rest) = encoded_value.split_at(1);
    let tag = tag.first().unwrap();
    eprintln!("tag: {}", tag);
    match tag {
        b'i' => {
            if let Some((number, rest)) = rest.split(|&b| b == b'e').next().and_then(|digits| {
                let n = std::str::from_utf8(digits).ok()?.parse::<i64>().ok()?;
                Some((n, &rest[digits.len() + 1..]))
            }) {
                eprintln!("number: {} rest: {:?}", number, rest);
                *remainder = rest;
                return serde_json::Value::Number(serde_json::Number::from(number));
            }
        }
        b'0'..=b'9' => {
            if let Some((len, rest)) = encoded_value.split(|&b| b == b':').next().and_then(|len| {
                let n = std::str::from_utf8(len).ok()?.parse::<usize>().ok()?;
                Some((n, &encoded_value[len.len() + 1..]))
            }) {
                *remainder = &rest[len..];
                eprintln!("len: {} rest: {:?}", len, rest);
                return serde_json::Value::String(std::str::from_utf8(&rest[..len]).unwrap().to_string());
            }
        }
        b'l' => {
            let mut list = Vec::new();
            let mut retval = rest;
            while retval[0] != b'e' && !retval.is_empty() {
                list.push(decode_bencoded_value(&retval, &mut retval));
                eprintln!("retval: {:?}", retval);
            }
            if retval.len() > 1 {
                *remainder = &retval[1..]; // Skip the 'e'
            } else {
                *remainder = &[]; // Handle case where retval is just "e"
            }
            eprint!("list {:?}", list);
            return serde_json::Value::Array(list);
        }
        b'd' => {
            let mut dict = serde_json::Map::new();
            let mut retval = rest;
            while retval[0] != b'e' && !retval.is_empty() {
                let key = decode_bencoded_value(&retval, &mut retval);
                let k = match key {
                    serde_json::Value::String(s) => s,
                    _ => panic!("Key is not a string"),
                };
                let value = decode_bencoded_value(&retval, &mut retval);
                dict.insert(k, value);
                eprintln!("retval: {:?}", retval);
            }
            if retval.len() > 1 {
                *remainder = &retval[1..]; // Skip the 'e'
            } else {
                *remainder = &[]; // Handle case where retval is just "e"
            }
            return dict.into();
        }
        _ => {
            panic!("Unhandled encoded value: {:?}", encoded_value);
        }
    }
    panic!("Unhandled encoded value: {:?}", encoded_value);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {

    let a = read_torrent_file("sample.torrent");
    
    let b = 0;
}
