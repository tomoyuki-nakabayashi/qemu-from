#![allow(non_snake_case)]

#[macro_use]
extern crate combine;
extern crate itertools;
use combine::{Parser, many1, token, sep_by, between, one_of};
use combine::char::{hex_digit, letter, spaces, string, newline};
use combine::parser::repeat::skip_until;

mod register_parser;

#[derive(Debug, PartialEq)]
pub(crate) struct GeneralRegister (String, u64);

#[derive(Debug, PartialEq)]
pub(crate) struct SegmentRegister (String, (u64, u64, u64, u64));

#[derive(Debug, PartialEq)]
struct SegmentRegisters {
    ES: u64,
    CS: u64,
    SS: u64,
    DS: u64,
    FS: u64,
    GS: u64,
}

#[derive(Debug, PartialEq)]
struct DescriptorTable {
    GDT: (u64, u64),
    IDT: (u64, u64),
}

#[derive(Debug, PartialEq)]
struct ControlRegs {
    CR0: u64,
    CR1: u64,
    CR2: u64,
    CR3: u64,
    DR0: u64,
    DR1: u64,
    DR2: u64,
    DR3: u64,
}

#[derive(Debug, PartialEq)]
struct StatusRegisters {
    EIP: GeneralRegister,
    EFLAGS_RAW: GeneralRegister,
    EFLAGS: Vec<char>,
    HFLAGS: Vec<GeneralRegister>,
}

#[derive(Debug, PartialEq)]
struct Cpu {
    regs: [u64; 8],
    status_regs: StatusRegisters,
    segment_regs: SegmentRegisters,
    dt: DescriptorTable,
    control_regs: ControlRegs,
    efer: u64,
}

// This will be removed.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CpuRegister {
    General(String, u32),
    Segment(String, (u64, u64, u64, u64)),
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
    use register_parser::{gpr_parser, eflags_parser};

    #[test]
    fn general_register() {
        assert_eq!(parse_general_register("EAX=0000aa55 EBX=00000000"),
            vec![CpuRegister::General("EAX".to_string(), 0x0000aa55u32),
                CpuRegister::General("EBX".to_string(), 0x00000000u32)]);
    }

    #[test]
    fn split_segment() {
        use itertools::Itertools;
        let id = many1::<String, _>(letter()).skip(spaces()).skip(token('='));
        let hex = many1::<String, _>(hex_digit())
            .map(|h| u64::from_str_radix(&h, 16).unwrap());
        let hex_list = sep_by::<Vec<u64>, _, _>(hex, spaces())
            .map(|hexes| hexes.into_iter().tuples::<(_,_,_,_)>().next().unwrap());

        let mut parser = (id, hex_list)
            .map(move |(id, d)| CpuRegister::Segment(id, d));

        let res = parser.parse("ES =0000 00000000 0000ffff 00009300");
        assert_eq!(res, Ok((CpuRegister::Segment("ES".to_string(), (0, 0, 0xffff, 0x9300)), "")));
    }

    #[test]
    fn status_registers() {
        let mut parser = struct_parser!{
            StatusRegisters {
                EIP: gpr_parser(),
                _: spaces(),
                EFLAGS_RAW: gpr_parser(),
                _: spaces(),
                EFLAGS: eflags_parser(),
                _: spaces(),
                HFLAGS: sep_by(gpr_parser(), spaces()),
            }
        };

        let res = parser.parse("EIP=00007c00 EFL=00000202 [-------] CPL=0 II=0 A20=1 SMM=0 HLT=0");
        assert_eq!(res.unwrap(), (StatusRegisters{
            EIP: gpr_parser().parse("EIP=00007c00").unwrap().0,
            EFLAGS_RAW: gpr_parser().parse("EFL=00000202").unwrap().0,
            EFLAGS: eflags_parser().parse("[-------]").unwrap().0,
            HFLAGS: sep_by(gpr_parser(), spaces()).parse("CPL=0 II=0 A20=1 SMM=0 HLT=0").unwrap().0 
            },
            ""));
    }

    #[test]
    fn eflags() {
        let eflag = many1::<Vec<_>, _>(one_of("DOSZAPC-".chars()));
        let mut parser = between(token('['), token(']'), eflag);

        let res = parser.parse("[-O----C]");
        assert_eq!(res.unwrap(), (vec!['-', 'O', '-', '-', '-', '-', 'C'], ""));
    }

    #[test]
    fn gdt() {
        let dt = string("GDT").or(string("IDT")).skip(token('='));
        let value = || spaces().with(many1::<String, _>(hex_digit())
            .map(|h| u64::from_str_radix(&h, 16).unwrap()));
        let value_pair = (value(), value());

        let mut parser = (dt, value_pair);
        let res = parser.parse("GDT=     000f6c00 00000037").unwrap();
        assert_eq!(res, (("GDT", (0xf6c00, 0x37)), ""));

        let res = parser.parse("IDT=     00000000 000003ff").unwrap();
        assert_eq!(res, (("IDT", (0, 0x3ff)), ""));
    }

    #[test]
    fn qemu_internal() {
        let mut parser = skip_until(newline());
        let res = parser.parse("CCS=00000000 CCD=0000fea4 CCO=EFLAGS\n").unwrap();

        assert_eq!(res, ((), "\n"));
    }
}