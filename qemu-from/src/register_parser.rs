use ::{GeneralRegister, SegmentRegister};
use combine::{Parser, Stream, many1, token, sep_by, one_of, between};
use combine::char::{hex_digit, letter, spaces, alpha_num};
use combine::error::ParseError;
extern crate itertools;

pub(crate) fn gpr_parser<I>() -> impl Parser<Input = I, Output = GeneralRegister>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = many1::<String, _>(alpha_num()).skip(token('='));
    let value = many1::<String, _>(hex_digit());
    let parser = (id, value)
        .map(|(id, value)| {
            GeneralRegister(id, u64::from_str_radix(&value, 16).unwrap())
        });

    parser
}

fn segment_parser<I>() -> impl Parser<Input = I, Output = SegmentRegister>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    use itertools::Itertools;
    let id = many1::<String, _>(letter()).skip(spaces()).skip(token('='));
    let hex = many1::<String, _>(hex_digit())
        .map(|h| u64::from_str_radix(&h, 16).unwrap());
    let hex_list = sep_by::<Vec<u64>, _, _>(hex, spaces())
        .map(|hexes| hexes.into_iter().tuples::<(_,_,_,_)>().next().unwrap());

    let parser = (id, hex_list)
        .map(move |(id, d)| SegmentRegister(id, d));

    parser
}

pub(crate) fn eflags_parser<I>() -> impl Parser<Input = I, Output = Vec<char>>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let eflag = many1::<Vec<char>, _>(one_of("DOSZAPC-".chars()));
    let parser = between(token('['), token(']'), eflag);

    parser
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_general_register() {
        let result = gpr_parser().parse("EAX=0000aa55");
        assert_eq!(result, Ok((GeneralRegister("EAX".to_string(), 0xaa55u64), "")));
    }

    #[test]
    fn get_segment_register() {
        let result = segment_parser().parse("ES =0000 00000000 0000ffff 00009300");
        assert_eq!(result, Ok((SegmentRegister("ES".to_string(), (0, 0, 0xffff, 0x9300)), "")));
    }
}