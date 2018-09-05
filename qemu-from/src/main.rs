extern crate combine;
use combine::{Parser, many1, token, sep_by};
use combine::char::{hex_digit, letter, alpha_num, spaces};

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

    let mut parser = sep_by(many1::<String, _>(letter().or(token('=')).or(alpha_num())), spaces());
    assert_eq!(parser.parse("EAX=0000aa55 EBX=00000000"), Ok((vec!["EAX=0000aa55".to_string(), "EBX=00000000".to_string()], "")));

    let mut register = sep_by::<Vec<String>, _, _>(many1::<String, _>(alpha_num()), token('='))
        .map(|reg| CpuRegister::General(reg[0].clone(), u32::from_str_radix(&reg[1], 16).unwrap()));
    assert_eq!(register.parse("EAX=0000aa55"), Ok((CpuRegister::General("EAX".to_string(), 0xaa55u32), "")));

    let mut id = many1::<String, _>(letter()).skip(token('='));
    let mut value = many1::<String, _>(hex_digit())
        .map(|value| u32::from_str_radix(&value, 16).unwrap());
    let res = id.parse("EAX=0000aa55")
        .map(|x| CpuRegister::General(x.0, value.parse(x.1).unwrap().0));
    assert_eq!(res, Ok(CpuRegister::General("EAX".to_string(), 0x0000aa55u32)));

    let id = many1::<String, _>(letter()).skip(token('='));
    let value = many1::<String, _>(hex_digit());
    let mut register = (id, value)
        .map(|(id, value)| {
            CpuRegister::General(id, u32::from_str_radix(&value, 16).unwrap())
        });

    assert_eq!(register.parse("EAX=0000aa55"), Ok((CpuRegister::General("EAX".to_string(), 0x0000aa55u32), "")));
}
