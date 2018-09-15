use ::{HFlag};
use combine::{Parser, Stream, many1, token, one_of, between, count};
use combine::parser::repeat::skip_until;
use combine::char::{hex_digit, letter, spaces, alpha_num, string};
use combine::error::ParseError;
extern crate itertools;

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
    let hex = spaces().with(many1::<String, _>(hex_digit()))
        .map(|h| u64::from_str_radix(&h, 16).unwrap());

    let hex_list = count::<Vec<u64>, _>(4, hex)
        .map(|hexes| hexes.into_iter().tuples::<(_,_,_,_)>().next().unwrap());
    let parser = (id, hex_list)
        .map(move |(_id, d)| d);

    parser
}

pub(crate) fn hflag_parser<I>() -> impl Parser<Input = I, Output = HFlag>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = spaces().with(many1::<String, _>(alpha_num()).skip(token('=')));
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

pub(crate) fn dt_parser<I>() -> impl Parser<Input = I, Output = (u64, u64)>
    where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = string("GDT").or(string("IDT")).skip(token('='));
    let value = || spaces().with(many1::<String, _>(hex_digit())
        .map(|h| u64::from_str_radix(&h, 16).unwrap()));
    let value_pair = (value(), value());

    let parser = (id, value_pair).map(|(_, pair)| pair);
    parser
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn hflags() {
        let res = count::<Vec<HFlag>, _>(5, hflag_parser()).parse("CPL=0 II=0 A20=1 SMM=0 HLT=0").unwrap().0;
        println!("{:?}", res);
    }
}