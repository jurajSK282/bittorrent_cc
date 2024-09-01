use hex::encode;
use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if let Some((len, rest)) = encoded_value.split_once(":") {
        if let Ok(len) = len.parse::<usize>() {
            return serde_json::Value::String(rest[..len].to_string());
        } else {
            panic!("Invalid length: {}", len);
        }
    } else if encoded_value.starts_with("i") && encoded_value.ends_with("e") {
        let number = &encoded_value[1..encoded_value.len() - 1];
        if let Ok(number) = number.parse::<i64>() {
            return serde_json::Value::Number(serde_json::Number::from(number));
        } else {
            panic!("Invalid number: {}", number);
        }
    }
    panic!("Unhandled encoded value: {}", encoded_value)
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        eprintln!("unknown command: {}", args[1])
    }
}
