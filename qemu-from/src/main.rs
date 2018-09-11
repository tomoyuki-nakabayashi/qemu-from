#![allow(non_snake_case)]

#[macro_use]
extern crate combine;
extern crate itertools;
use combine::{Parser, many1, token, sep_by, between, one_of};
use combine::char::{hex_digit, letter, spaces, string, newline};
use combine::parser::repeat::skip_until;

mod register_parser;

#[derive(Debug, PartialEq)]
pub(crate) struct GeneralRegister (usize, u64);

#[derive(Debug, PartialEq)]
pub(crate) struct SegmentRegister (String, (u64, u64, u64, u64));

#[derive(Debug, PartialEq)]
pub(crate) struct HFlag (String, u64);

#[derive(Debug, PartialEq)]
struct SegmentRegisters {
    ES: (u64, u64, u64, u64),
    CS: (u64, u64, u64, u64),
    SS: (u64, u64, u64, u64),
    DS: (u64, u64, u64, u64),
    FS: (u64, u64, u64, u64),
    GS: (u64, u64, u64, u64),
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
    EIP: u64,
    EFLAGS_RAW: u64,
    EFLAGS: Vec<char>,
    HFLAGS: Vec<HFlag>,
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

fn main() {
}


#[cfg(test)]
mod test {
    use super::*;
    use register_parser::{gpr_parser, eflags_parser, hflag_parser, qword_parser, segment_parser};

    #[test]
    fn status_registers() {
        let mut parser = struct_parser!{
            StatusRegisters {
                EIP: qword_parser(),
                EFLAGS_RAW: qword_parser(),
                EFLAGS: eflags_parser(),
                _: spaces(),
                HFLAGS: sep_by(hflag_parser(), spaces()),
            }
        };

        let res = parser.parse("EIP=00007c00 EFL=00000202 [-------] CPL=0 II=0 A20=1 SMM=0 HLT=0");
        assert_eq!(res.unwrap(), (StatusRegisters{
            EIP: qword_parser().parse("EIP=00007c00").unwrap().0,
            EFLAGS_RAW: qword_parser().parse("EFL=00000202").unwrap().0,
            EFLAGS: eflags_parser().parse("[-------]").unwrap().0,
            HFLAGS: sep_by(hflag_parser(), spaces()).parse("CPL=0 II=0 A20=1 SMM=0 HLT=0").unwrap().0 
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

    #[test]
    fn segments() {
        let mut parser = segment_parser();
        let res = parser.parse("ES =0000 00000000 0000ffff 00009300\n").unwrap();

        let mut parser = many1::<String, _>(hex_digit()).map(|h| u64::from_str_radix(&h, 16).unwrap());
        let res = parser.parse("9300\n").unwrap();
/* 
        let mut parser = struct_parser!{
            SegmentRegisters {
                ES: segment_parser().skip(newline()),
                CS: segment_parser().skip(newline()),
                SS: segment_parser().skip(newline()),
                DS: segment_parser().skip(newline()),
                FS: segment_parser().skip(newline()),
                GS: segment_parser(),
            }
        };

        let res = parser.parse("ES =0000 00000000 0000ffff 00009300\nCS =0000 00000000 0000ffff 00009b00\nSS =0000 00000000 0000ffff 00009300\nDS =0000 00000000 0000ffff 00009300\nFS =0000 00000000 0000ffff 00009300\nGS =0000 00000000 0000ffff 00009300");
 */
    }
}