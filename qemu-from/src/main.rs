#![allow(non_snake_case)]

#[macro_use]
extern crate combine;
extern crate itertools;
use combine::{Parser, Stream, count};
use combine::char::{spaces, newline};
use combine::parser::repeat::{skip_until};
use combine::error::ParseError;

mod register_parser;

use register_parser::{
        eflags_parser, hflag_parser, qword_parser, segment_parser, dt_parser};

#[derive(Debug, PartialEq)]
pub(crate) struct GeneralRegister (usize, u64);

#[derive(Debug, PartialEq)]
pub(crate) struct SegmentRegister (String, (u64, u64, u64, u64));

#[derive(Debug, PartialEq)]
pub(crate) struct HFlag (String, u64);

#[derive(Debug, PartialEq)]
struct GeneralRegisters {
    EAX: u64,
    EBX: u64,
    ECX: u64,
    EDX: u64,
    ESI: u64,
    EDI: u64,
    EBP: u64,
    ESP: u64,
}

fn general_regs_parser<I>() -> impl Parser<Input = I, Output = GeneralRegisters>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        GeneralRegisters {
            EAX: qword_parser(),
            EBX: qword_parser(),
            ECX: qword_parser(),
            EDX: qword_parser().skip(newline()),
            ESI: qword_parser(),
            EDI: qword_parser(),
            EBP: qword_parser(),
            ESP: qword_parser().skip(newline()),
        }
    };

    parser
}

#[derive(Debug, PartialEq)]
struct SegmentRegisters {
    ES: (u64, u64, u64, u64),
    CS: (u64, u64, u64, u64),
    SS: (u64, u64, u64, u64),
    DS: (u64, u64, u64, u64),
    FS: (u64, u64, u64, u64),
    GS: (u64, u64, u64, u64),
    LDT: (u64, u64, u64, u64),
    TR: (u64, u64, u64, u64),
}

fn segment_regs_parser<I>() -> impl Parser<Input = I, Output = SegmentRegisters>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        SegmentRegisters {
            ES: segment_parser().skip(newline()),
            CS: segment_parser().skip(newline()),
            SS: segment_parser().skip(newline()),
            DS: segment_parser().skip(newline()),
            FS: segment_parser().skip(newline()),
            GS: segment_parser().skip(newline()),
            LDT: segment_parser().skip(newline()),
            TR: segment_parser().skip(newline()),
        }
    };

    parser
}

#[derive(Debug, PartialEq)]
struct DescriptorTable {
    GDT: (u64, u64),
    IDT: (u64, u64),
}

fn descriptor_tables_parser<I>() -> impl Parser<Input = I, Output = DescriptorTable>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        DescriptorTable {
            GDT: dt_parser().skip(newline()),
            IDT: dt_parser().skip(newline()),
        }
    };

    parser
}


#[derive(Debug, PartialEq)]
struct ControlRegs {
    CR0: u64,
    CR1: u64,
    CR2: u64,
    CR3: u64,
}

fn control_regs_parser<I>() -> impl Parser<Input = I, Output = ControlRegs>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        ControlRegs {
            CR0: qword_parser(),
            CR1: qword_parser(),
            CR2: qword_parser(),
            CR3: qword_parser().skip(newline()),
        }
    };

    parser
}

#[derive(Debug, PartialEq)]
struct DebugRegs {
    DR0: u64,
    DR1: u64,
    DR2: u64,
    DR3: u64,
    DR6: u64,
    DR7: u64,
}

fn debug_regs_parser<I>() -> impl Parser<Input = I, Output = DebugRegs>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        DebugRegs {
            DR0: qword_parser(),
            DR1: qword_parser(),
            DR2: qword_parser(),
            DR3: qword_parser().skip(newline()),
            DR6: qword_parser(),
            DR7: qword_parser().skip(newline()),
        }
    };

    parser
}

#[derive(Debug, PartialEq)]
struct StatusRegisters {
    EIP: u64,
    EFLAGS_RAW: u64,
    EFLAGS: Vec<char>,
    HFLAGS: Vec<HFlag>,
}

fn status_regs_parser<I>() -> impl Parser<Input = I, Output = StatusRegisters>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        StatusRegisters {
            EIP: qword_parser(),
            EFLAGS_RAW: qword_parser(),
            EFLAGS: eflags_parser(),
            _: spaces(),
            HFLAGS: count::<Vec<HFlag>, _>(5, hflag_parser()),
            _: newline(),
        }
    };

    parser
}

#[derive(Debug, PartialEq)]
struct Cpu {
    regs: GeneralRegisters,
    status_regs: StatusRegisters,
    segment_regs: SegmentRegisters,
    dt: DescriptorTable,
    control_regs: ControlRegs,
    debug_regs: DebugRegs,
    efer: u64,
}

fn cpu_state_parser<I>() -> impl Parser<Input = I, Output = Cpu>
    where I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let parser = struct_parser!{
        Cpu {
            regs: general_regs_parser(),
            status_regs: status_regs_parser(),
            segment_regs: segment_regs_parser(),
            dt: descriptor_tables_parser(),
            control_regs: control_regs_parser(),
            debug_regs: debug_regs_parser(),
            _: skip_until(newline()).skip(newline()),
            efer: qword_parser(),
        }
    };

    parser
}

fn main() {
    let mut parser = cpu_state_parser();
    let res = parser.parse("EAX=0000aa55 EBX=00000000 ECX=00000000 EDX=00000080
ESI=00000000 EDI=00000000 EBP=00000000 ESP=00006f2c
EIP=00007c00 EFL=00000202 [-------] CPL=0 II=0 A20=1 SMM=0 HLT=0
ES =0000 00000000 0000ffff 00009300
CS =0000 00000000 0000ffff 00009b00
SS =0000 00000000 0000ffff 00009300
DS =0000 00000000 0000ffff 00009300
FS =0000 00000000 0000ffff 00009300
GS =0000 00000000 0000ffff 00009300
LDT=0000 00000000 0000ffff 00008200
TR =0000 00000000 0000ffff 00008b00
GDT=     000f6c00 00000037
IDT=     00000000 000003ff
CR0=00000010 CR2=00000000 CR3=00000000 CR4=00000000
DR0=0000000000000000 DR1=0000000000000000 DR2=0000000000000000 DR3=0000000000000000
DR6=00000000ffff0ff0 DR7=0000000000000400
CCS=00000000 CCD=0000fea4 CCO=EFLAGS
EFER=0000000000000000"
    );

    println!("{:?}", res);
}


#[cfg(test)]
mod test {
    use super::*;
    use register_parser::{eflags_parser, hflag_parser, qword_parser};

    #[test]
    fn general_regs() {
        let mut parser = general_regs_parser();
        let res = parser.parse("EAX=0000aa55 EBX=00000000 ECX=00000000 EDX=00000080\nESI=00000000 EDI=00000000 EBP=00000000 ESP=00006f2c\n").unwrap();
        let expect = GeneralRegisters { EAX: 0xaa55, EBX: 0, ECX: 0, EDX: 0x80, ESI: 0, EDI: 0, EBP: 0, ESP: 0x6f2c };
        assert_eq!(res, (expect, ""));
    }

    #[test]
    fn status_registers() {
        let mut parser = status_regs_parser();
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
    fn descriptor_tables() {
        let mut parser = descriptor_tables_parser();
        let res = parser.parse("GDT=     000f6c00 00000037\nIDT=     00000000 000003ff\n").unwrap();
        let expect = DescriptorTable { GDT: (0xf6c00, 0x37), IDT: (0x0, 0x3ff) };
        assert_eq!(res, (expect, ""));
    }

    #[test]
    fn control_regs() {
        let mut parser = control_regs_parser();
        let res = parser.parse("CR0=00000010 CR2=00000000 CR3=00000000 CR4=00000000\n").unwrap();
        let expect = ControlRegs { CR0: 0x10, CR1: 0, CR2: 0, CR3: 0, };
        assert_eq!(res, (expect, ""));
    }

    #[test]
    fn debug_regs() {
        let mut parser = debug_regs_parser();
        let res = parser.parse("DR0=0000000000000000 DR1=0000000000000000 DR2=0000000000000000 DR3=0000000000000000\nDR6=00000000ffff0ff0 DR7=0000000000000400\n").unwrap();
        let expect = DebugRegs { DR0: 0, DR1: 0, DR2: 0, DR3: 0, DR6: 0xffff0ff0, DR7: 0x400 };
        assert_eq!(res, (expect, ""));
    }

    #[test]
    fn qemu_internal() {
        let mut parser = skip_until(newline()).skip(newline());
        let res = parser.parse("CCS=00000000 CCD=0000fea4 CCO=EFLAGS\n").unwrap();

        assert_eq!(res, ((), ""));
    }

    #[test]
    fn segment_regs() {
        let mut parser = segment_regs_parser();
        let res = parser.parse("ES =0000 00000000 0000ffff 00009300\nCS =0000 00000000 0000ffff 00009b00\nSS =0000 00000000 0000ffff 00009300\nDS =0000 00000000 0000ffff 00009300\nFS =0000 00000000 0000ffff 00009300\nGS =0000 00000000 0000ffff 00009300\nLDT=0000 00000000 0000ffff 00008200\nTR =0000 00000000 0000ffff 00008b00\n");
        let expect = SegmentRegisters {
            ES: (0, 0, 0xffff, 0x9300),
            CS: (0, 0, 0xffff, 0x9b00),
            SS: (0, 0, 0xffff, 0x9300),
            DS: (0, 0, 0xffff, 0x9300),
            FS: (0, 0, 0xffff, 0x9300),
            GS: (0, 0, 0xffff, 0x9300),
            LDT: (0, 0, 0xffff, 0x8200),
            TR: (0, 0, 0xffff, 0x8b00),
        };
        assert_eq!(res, Ok((expect, "")));
    }
}