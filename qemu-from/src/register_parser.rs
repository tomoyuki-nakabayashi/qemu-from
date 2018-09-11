use ::{GeneralRegister, SegmentRegister, HFlag};
use combine::{Parser, Stream, many1, token, sep_by, one_of, between};
use combine::parser::repeat::skip_until;
use combine::char::{hex_digit, letter, spaces, alpha_num};
use combine::error::ParseError;
extern crate itertools;

fn reg_number_from_id(id: &str) -> Option<usize> {
    match id {
        "EAX" => Some(0),
        "EBX" => Some(3),
        "ECX" => Some(1),
        "EDX" => Some(2),
        "ESI" => Some(4),
        "EDI" => Some(5),
        "EBP" => Some(6),
        "ESP" => Some(7),
        &_ => None
    }
}

pub(crate) fn gpr_parser<I>() -> impl Parser<Input = I, Output = GeneralRegister>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = many1::<String, _>(alpha_num()).skip(token('='));
    let value = many1::<String, _>(hex_digit());
    let parser = (id, value)
        .map(|(id, value)| {
            let id = reg_number_from_id(&id).expect("Invalid register id.");
            let value = u64::from_str_radix(&value, 16).expect("Fail to convert to u64");
            GeneralRegister(id, value)
        });

    parser
}

pub(crate) fn qword_parser<I>() -> impl Parser<Input = I, Output = u64>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let _id = spaces().with(skip_until(token('=')).skip(token('=')));
    let value = many1::<String, _>(hex_digit());
    let parser = (_id, value)
        .map(|(_id, value)| {
            let value = u64::from_str_radix(&value, 16).expect("Fail to convert to u64");
            value
        });

    parser
}

pub(crate) fn segment_parser<I>() -> impl Parser<Input = I, Output = (u64, u64, u64, u64)>
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
        .map(move |(id, d)| d);

    parser
}

pub(crate) fn hflag_parser<I>() -> impl Parser<Input = I, Output = HFlag>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = many1::<String, _>(alpha_num()).skip(token('='));
    let value = many1::<String, _>(hex_digit()).map(|h| u64::from_str_radix(&h, 16).unwrap());
    let parser = (id, value).map(|(id, value)| { HFlag(id, value) });

    parser
}

pub(crate) fn eflags_parser<I>() -> impl Parser<Input = I, Output = Vec<char>>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let eflag = many1::<Vec<char>, _>(one_of("DOSZAPC-".chars()));
    let parser = spaces().with(between(token('['), token(']'), eflag));

    parser
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_general_register() {
        let result = gpr_parser().parse("EAX=0000aa55");
        assert_eq!(result, Ok((GeneralRegister(0 as usize, 0xaa55u64), "")));
    }

    #[test]
    fn get_segment_register() {
        let result = segment_parser().parse("ES =0000 00000000 0000ffff 00009300");
        assert_eq!(result, Ok(((0, 0, 0xffff, 0x9300), "")));
    }

    #[test]
    fn qword() {
        let result = qword_parser().parse("EIP=00007c00");
        assert_eq!(result, Ok((0x7c00, "")));
    }
}