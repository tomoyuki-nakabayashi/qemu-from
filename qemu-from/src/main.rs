extern crate combine;
use combine::{Parser, many1, token};
use combine::char::{hex_digit};

fn main() {
    let mut parser = token('=')
        .with(many1(hex_digit()).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    assert_eq!(parser.parse("=0000aa55"), Ok((0x0000aa55u32, "")));
}
