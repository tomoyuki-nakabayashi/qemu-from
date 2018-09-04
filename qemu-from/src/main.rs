extern crate combine;
use combine::{Parser, many1, token, sep_by};
use combine::char::{hex_digit, letter, alpha_num};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CpuRegister {
    General(String, u32),
}

fn main() {
    let mut parser = token('=')
        .with(many1(hex_digit()).map(|s: String| u32::from_str_radix(&s, 16).unwrap()));
    assert_eq!(parser.parse("=0000aa55"), Ok((0x0000aa55u32, "")));

    let mut parser = sep_by::<Vec<String>, _, _>(many1::<String, _>(alpha_num()), token('='));
    assert_eq!(parser.parse("EAX=0000aa55"), Ok((vec!["EAX".to_string(), "0000aa55".to_string()], "")));
}
