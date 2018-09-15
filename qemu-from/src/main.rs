#![allow(non_snake_case)]

#[macro_use]
extern crate combine;
extern crate itertools;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod register_parser;
mod cpu_state_parser;

use combine::{Parser};

fn main() {
    let mut parser = cpu_state_parser::cpu_state_parser();
    let res = parser.parse(
        "EAX=0000aa55 EBX=00000000 ECX=00000000 EDX=00000080
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
EFER=0000000000000000",
    );

    let json_str = serde_json::to_string(&res.unwrap().0).unwrap();
    println!("{}", json_str);
}
