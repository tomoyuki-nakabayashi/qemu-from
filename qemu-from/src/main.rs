extern crate combine;
extern crate itertools;
use combine::{Parser, many1, token, sep_by};
use combine::char::{hex_digit, letter, spaces};

mod register_parser;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CpuRegister {
    General(String, u32),
    Segment(String, u64, u64, u64, u64),
}

fn main() {
}

fn parse_segment_register(line: &str) -> CpuRegister {
    CpuRegister::Segment("ES".to_string(), 0, 0, 0xffff, 0x9300)
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

    #[test]
    fn segment_register() {
        assert_eq!(parse_segment_register("ES =0000 00000000 0000ffff 00009300"),
            CpuRegister::Segment("ES".to_string(), 0, 0, 0xffff, 0x9300));
    }

    #[test]
    fn split_segment() {
        let id = many1::<String, _>(letter()).skip(spaces()).skip(token('='));
        let hex = many1::<String, _>(hex_digit())
            .map(|h| u64::from_str_radix(&h, 16).unwrap());
        let hex_list = sep_by::<Vec<u64>, _, _>(hex, spaces());

        let mut parser = (id, hex_list);
        assert_eq!(parser.parse("ES =0000 00000000 0000ffff 00009300"),
            Ok((("ES".to_string(), vec![0, 0, 0xffff, 0x9300]), "")));

        use itertools::Itertools;
        let mut t = vec![0u64, 0, 0xffff, 0x9300].into_iter().tuples::<(_, _, _, _)>();
        assert_eq!(t.next(), Some((0u64, 0, 0xffff, 0x9300)));
    }
}