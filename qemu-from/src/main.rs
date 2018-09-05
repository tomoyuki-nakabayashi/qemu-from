extern crate combine;
use combine::{Parser, many1, token, sep_by};
use combine::char::{hex_digit, letter, spaces};

#[derive(Debug, Clone, PartialEq, Eq)]
enum CpuRegister {
    General(String, u32),
}

fn main() {
}

fn parse_general_register(line: &str) -> Vec<CpuRegister> {
    let id = many1::<String, _>(letter()).skip(token('='));
    let value = many1::<String, _>(hex_digit());
    let register = (id, value)
        .map(|(id, value)| {
            CpuRegister::General(id, u32::from_str_radix(&value, 16).unwrap())
        });

    let mut parser = sep_by::<Vec<CpuRegister>, _, _>(register, spaces());
    parser.parse(line).unwrap().0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn general_register() {
        assert_eq!(parse_general_register("EAX=0000aa55 EBX=00000000"),
            vec![CpuRegister::General("EAX".to_string(), 0x0000aa55u32),
                CpuRegister::General("EBX".to_string(), 0x00000000u32)]);
    }
}