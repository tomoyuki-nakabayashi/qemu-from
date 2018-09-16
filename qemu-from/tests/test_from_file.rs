extern crate qemu_from;
extern crate combine;
extern crate serde_json;

use qemu_from::x86_cpu_state_parser::x86_cpu_state_parser;
use combine::Parser;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[test]
fn test_from_file() {
    let file = File::open("./tests/qemu.log").expect("cannot open file");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("fail to read from file.");

    let mut parser = x86_cpu_state_parser();
    let res = parser.parse(&buffer[..]);

    assert!(res.is_ok());
    let json_str = serde_json::to_string(&res.unwrap().0);
    assert!(json_str.is_ok());
    println!("{}", json_str.unwrap());
}

#[test]
fn test_from_file_as_u8() {
    let file = File::open("./tests/qemu.log").expect("cannot open file");
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).expect("fail to read from file.");
    let buffer = std::str::from_utf8(&bytes).unwrap();

    let mut parser = x86_cpu_state_parser();
    let res = parser.parse(buffer);

    assert!(res.is_ok());
    let json_str = serde_json::to_string(&res.unwrap().0);
    assert!(json_str.is_ok());
    println!("{}", json_str.unwrap());
}

#[test]
fn test_from_file_loop() {
    let file = File::open("./tests/qemu_lines.log").expect("cannot open file");
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).expect("fail to read from file.");
    let mut buffer = std::str::from_utf8(&bytes).unwrap();
    let mut parser = x86_cpu_state_parser();

    let mut count = 0;
    while let Ok((result, remaining)) = parser.parse(buffer) {
        let json_str = serde_json::to_string(&result);
        assert!(json_str.is_ok());
        println!("{}", json_str.unwrap());
        buffer = remaining;
        count += 1;
    }

    assert_eq!(count, 2);
}