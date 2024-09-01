use hex::encode;
use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value<'a>(encoded_value: &'a str, remainder: &mut &'a str) -> serde_json::Value {
    let (tag, rest) = encoded_value.split_at(1);
    let tag = tag.chars().next().unwrap();
    eprintln!("tag: {}", tag);
    match tag {
        'i' => {
            if let Some((number, rest)) = rest.split_once('e').and_then(|(digits, rest)| {
                let n = digits.parse::<i64>().ok()?;
                Some((n, rest))
            }) {
                eprintln!("number: {} rest: {}", number, rest);
                *remainder = rest;
                return serde_json::Value::Number(serde_json::Number::from(number));
            }
        }
        '0'..='9' => {
            if let Some((len, rest)) = encoded_value.split_once(':').and_then(|(len, rest)| {
                let n = len.parse::<usize>().ok()?;
                Some((n, rest))
            }) {
                *remainder = &rest[len..];
                eprintln!("len: {} rest: {}", len, rest);
                return serde_json::Value::String(rest[..len].to_string());
            }
        }
        'l' => {
            let mut list = Vec::new();
            let mut retval = rest;
            while retval.chars().next() != Some('e') && !retval.is_empty() {
                list.push(decode_bencoded_value(retval, &mut retval));
                eprintln!("retval: {}", retval);
            }
            if retval.len() > 1 {
                *remainder = &retval[1..]; // Skip the 'e'
            } else {
                *remainder = ""; // Handle case where retval is just "e"
            }
            eprint!("list {:?}", list);
            return serde_json::Value::Array(list);
        }
        _ => {
            panic!("Unhandled encoded value: {}", encoded_value);
        }
    }
    panic!("Unhandled encoded value: {}", encoded_value);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {

    let a = "li25el3:fooi-45ee5:helloe";
    let decoded = decode_bencoded_value(a, &mut "");
    eprint!("decoded: {}", decoded);
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        //let encoded_value = &args[2];
        //let decoded_value = decode_bencoded_value(encoded_value, &mut String::new());
        //println!("{}", decoded_value.to_string());
    } else {
        eprintln!("unknown command: {}", args[1])
    }
}
