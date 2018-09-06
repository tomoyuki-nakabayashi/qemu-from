use combine::{Parser, Stream, many1, token};
use combine::char::{hex_digit, letter};
use combine::error::ParseError;

#[derive(Debug, PartialEq)]
struct GeneralRegister (String, u64);

fn parse_register<I>() -> impl Parser<Input = I, Output = GeneralRegister>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let id = many1::<String, _>(letter()).skip(token('='));
    let value = many1::<String, _>(hex_digit());
    let parser = (id, value)
        .map(|(id, value)| {
            GeneralRegister(id, u64::from_str_radix(&value, 16).unwrap())
        });

    parser
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_general_register() {
        let result = parse_register().parse("EAX=0000aa55");
        assert_eq!(result, Ok((GeneralRegister("EAX".to_string(), 0xaa55u64), "")));
    }
}