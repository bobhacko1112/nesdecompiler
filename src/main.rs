use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
//opcode,mnemonic,addressing mode,bytes,cycles,flags
#[derive(Debug)]
struct Code <'a>{
    code: u8,
    mnemonic: &'a str,
    mode: &'a str,
    length: u8,
    cycles: u8,
    flags: &'a str
}
static offset:u8 = 16;
fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() !=2 {
        println!("Usage: $ nes_dc <nes_rom_filename.rom>");
        return Ok(());
    }
    args.next();
    let mut f = File::open(args.next().unwrap())?;


let mut each_byte = f.bytes();
for _ in 0..4 {
    each_byte.next();
}

let prg_count = each_byte.next().unwrap().unwrap();
let chr_count = each_byte.next().unwrap().unwrap();
for _ in 0..10 {
    each_byte.next();
}
println!("prg page count: 0x{:x} | chr page count 0x{:x}", prg_count, chr_count);
for page in 0..prg_count {
 //   println!("\n//PRG Page #{}", page);
for _ in 0..16383 {
    let this_byte = each_byte.next().unwrap().unwrap();
    for i in 0..key.len() {
        if key[i].code == this_byte{
            print!("\n{}", key[i].mnemonic);
            for j in 1..key[i].length {
                print!("{} 0x{:x}", if j ==2 {","} else {""}, each_byte.next().unwrap().unwrap());
            }
            print!(";");
            break;
        }
    }
}
}
for page in 0..chr_count {
    for i in 0..8191 {
        let this_byte = each_byte.next().unwrap().unwrap();

    }
}

    Ok(())
}

static key: &'static [Code] = &[
Code{ code: 0x69,mnemonic: "ADC",mode: "IMM",length: 2,cycles: 2,flags: "CZidbVN"},
Code{ code: 0x65,mnemonic: "ADC",mode: "ZP",length: 2,cycles: 3,flags: "CZidbVN"},
Code{ code: 0x75,mnemonic: "ADC",mode: "ZPX",length: 2,cycles: 4,flags: "CZidbVN"},
Code{ code: 0x6d,mnemonic: "ADC",mode: "ABS",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0x7d,mnemonic: "ADC",mode: "ABSX",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0x79,mnemonic: "ADC",mode: "ABSY",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0x61,mnemonic: "ADC",mode: "INDX",length: 2,cycles: 6,flags: "CZidbVN"},
Code{ code: 0x71,mnemonic: "ADC",mode: "INDY",length: 2,cycles: 5,flags: "CZidbVN"},
Code{ code: 0x29,mnemonic: "AND",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x25,mnemonic: "AND",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0x35,mnemonic: "AND",mode: "ZPX",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x2d,mnemonic: "AND",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x3d,mnemonic: "AND",mode: "ABSX",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x39,mnemonic: "AND",mode: "ABSY",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x21,mnemonic: "AND",mode: "INDX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0x31,mnemonic: "AND",mode: "INDY",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0x0a,mnemonic: "ASL",mode: "ACC",length: 1,cycles: 2,flags: "CZidbvN"},
Code{ code: 0x06,mnemonic: "ASL",mode: "ZP",length: 2,cycles: 5,flags: "CZidbvN"},
Code{ code: 0x16,mnemonic: "ASL",mode: "ZPX",length: 2,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x0e,mnemonic: "ASL",mode: "ABS",length: 3,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x1e,mnemonic: "ASL",mode: "ABSX",length: 3,cycles: 7,flags: "CZidbvN"},
Code{ code: 0x90,mnemonic: "BCC",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0xB0,mnemonic: "BCS",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0xF0,mnemonic: "BEQ",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0x30,mnemonic: "BMI",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0xD0,mnemonic: "BNE",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0x10,mnemonic: "BPL",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0x50,mnemonic: "BVC",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0x70,mnemonic: "BVS",mode: "REL",length: 2,cycles: 2/3,flags: "czidbvn"},
Code{ code: 0x24,mnemonic: "BIT",mode: "ZP",length: 2,cycles: 3,flags: "cZidbVN"},
Code{ code: 0x2c,mnemonic: "BIT",mode: "ABS",length: 3,cycles: 4,flags: "cZidbVN"},
Code{ code: 0x00,mnemonic: "BRK",mode: "IMP",length: 1,cycles: 7,flags: "czidbvn"},
Code{ code: 0x18,mnemonic: "CLC",mode: "IMP",length: 1,cycles: 2,flags: "Czidbvn"},
Code{ code: 0xd8,mnemonic: "CLD",mode: "IMP",length: 1,cycles: 2,flags: "cziDbvn"},
Code{ code: 0x58,mnemonic: "CLI",mode: "IMP",length: 1,cycles: 2,flags: "czIdbvn"},
Code{ code: 0xb8,mnemonic: "CLV",mode: "IMP",length: 1,cycles: 2,flags: "czidbVn"},
Code{ code: 0xea,mnemonic: "NOP",mode: "IMP",length: 1,cycles: 2,flags: "czidbvn"},
Code{ code: 0x48,mnemonic: "PHA",mode: "IMP",length: 1,cycles: 3,flags: "czidbvn"},
Code{ code: 0x68,mnemonic: "PLA",mode: "IMP",length: 1,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x08,mnemonic: "PHP",mode: "IMP",length: 1,cycles: 3,flags: "czidbvn"},
Code{ code: 0x28,mnemonic: "PLP",mode: "IMP",length: 1,cycles: 4,flags: "CZIDBVN"},
Code{ code: 0x40,mnemonic: "RTI",mode: "IMP",length: 1,cycles: 6,flags: "czidbvn"},
Code{ code: 0x60,mnemonic: "RTS",mode: "IMP",length: 1,cycles: 6,flags: "czidbvn"},
Code{ code: 0x38,mnemonic: "SEC",mode: "IMP",length: 1,cycles: 2,flags: "Czidbvn"},
Code{ code: 0xf8,mnemonic: "SED",mode: "IMP",length: 1,cycles: 2,flags: "cziDbvn"},
Code{ code: 0x78,mnemonic: "SEI",mode: "IMP",length: 1,cycles: 2,flags: "czIdbvn"},
Code{ code: 0xaa,mnemonic: "TAX",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x8a,mnemonic: "TXA",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xa8,mnemonic: "TAY",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x98,mnemonic: "TYA",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xba,mnemonic: "TSX",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x9a,mnemonic: "TXS",mode: "IMP",length: 1,cycles: 2,flags: "czidbvn"},
Code{ code: 0xc9,mnemonic: "CMP",mode: "IMM",length: 2,cycles: 2,flags: "CZidbvN"},
Code{ code: 0xc5,mnemonic: "CMP",mode: "ZP",length: 2,cycles: 3,flags: "CZidbvN"},
Code{ code: 0xd5,mnemonic: "CMP",mode: "ZPX",length: 2,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xcd,mnemonic: "CMP",mode: "ABS",length: 3,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xdd,mnemonic: "CMP",mode: "ABSX",length: 3,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xd9,mnemonic: "CMP",mode: "ABSY",length: 3,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xc1,mnemonic: "CMP",mode: "INDX",length: 2,cycles: 6,flags: "CZidbvN"},
Code{ code: 0xd1,mnemonic: "CMP",mode: "INDY",length: 2,cycles: 5,flags: "CZidbvN"},
Code{ code: 0xe0,mnemonic: "CPX",mode: "IMM",length: 2,cycles: 2,flags: "CZidbvN"},
Code{ code: 0xe4,mnemonic: "CPX",mode: "ZP",length: 2,cycles: 3,flags: "CZidbvN"},
Code{ code: 0xec,mnemonic: "CPX",mode: "ABS",length: 3,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xc0,mnemonic: "CPY",mode: "IMM",length: 2,cycles: 2,flags: "CZidbvN"},
Code{ code: 0xc4,mnemonic: "CPY",mode: "ZP",length: 2,cycles: 3,flags: "CZidbvN"},
Code{ code: 0xcc,mnemonic: "CPY",mode: "ABS",length: 3,cycles: 4,flags: "CZidbvN"},
Code{ code: 0xc6,mnemonic: "DEC",mode: "ZP",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0xd6,mnemonic: "DEC",mode: "ZPX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0xce,mnemonic: "DEC",mode: "ABS",length: 3,cycles: 6,flags: "cZidbvN"},
Code{ code: 0xde,mnemonic: "DEC",mode: "ABSX",length: 3,cycles: 7,flags: "cZidbvN"},
Code{ code: 0xca,mnemonic: "DEX",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x88,mnemonic: "DEY",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xe8,mnemonic: "INX",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xc8,mnemonic: "INY",mode: "IMP",length: 1,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x49,mnemonic: "EOR",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x45,mnemonic: "EOR",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0x55,mnemonic: "EOR",mode: "ZPX",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x4d,mnemonic: "EOR",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x5d,mnemonic: "EOR",mode: "ABSX",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x59,mnemonic: "EOR",mode: "ABSY",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x41,mnemonic: "EOR",mode: "INDX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0x51,mnemonic: "EOR",mode: "INDY",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0xe6,mnemonic: "INC",mode: "ZP",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0xf6,mnemonic: "INC",mode: "ZPX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0xee,mnemonic: "INC",mode: "ABS",length: 3,cycles: 6,flags: "cZidbvN"},
Code{ code: 0xfe,mnemonic: "INC",mode: "ABSX",length: 3,cycles: 7,flags: "cZidbvN"},
Code{ code: 0x4c,mnemonic: "JMP",mode: "ABS",length: 3,cycles: 3,flags: "czidbvn"},
Code{ code: 0x6c,mnemonic: "JMP",mode: "IND",length: 3,cycles: 5,flags: "czidbvn"},
Code{ code: 0x20,mnemonic: "JSR",mode: "ABS",length: 3,cycles: 6,flags: "czidbvn"},
Code{ code: 0xa9,mnemonic: "LDA",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xa5,mnemonic: "LDA",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0xb5,mnemonic: "LDA",mode: "ZPX",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xad,mnemonic: "LDA",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xbd,mnemonic: "LDA",mode: "ABSX",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xb9,mnemonic: "LDA",mode: "ABSY",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xa1,mnemonic: "LDA",mode: "INDX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0xb1,mnemonic: "LDA",mode: "INDY",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0xa2,mnemonic: "LDX",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xa6,mnemonic: "LDX",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0xb6,mnemonic: "LDX",mode: "ZPY",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xae,mnemonic: "LDX",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xbe,mnemonic: "LDX",mode: "ABSY",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xa0,mnemonic: "LDY",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0xa4,mnemonic: "LDY",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0xb4,mnemonic: "LDY",mode: "ZPX",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xac,mnemonic: "LDY",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0xbc,mnemonic: "LDY",mode: "ABSX",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x4a,mnemonic: "LSR",mode: "ACC",length: 1,cycles: 2,flags: "CZidbvN"},
Code{ code: 0x46,mnemonic: "LSR",mode: "ZP",length: 2,cycles: 5,flags: "CZidbvN"},
Code{ code: 0x56,mnemonic: "LSR",mode: "ZPX",length: 2,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x4e,mnemonic: "LSR",mode: "ABS",length: 3,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x5e,mnemonic: "LSR",mode: "ABSX",length: 3,cycles: 7,flags: "CZidbvN"},
Code{ code: 0x09,mnemonic: "ORA",mode: "IMM",length: 2,cycles: 2,flags: "cZidbvN"},
Code{ code: 0x05,mnemonic: "ORA",mode: "ZP",length: 2,cycles: 3,flags: "cZidbvN"},
Code{ code: 0x15,mnemonic: "ORA",mode: "ZPX",length: 2,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x0d,mnemonic: "ORA",mode: "ABS",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x1d,mnemonic: "ORA",mode: "ABSX",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x19,mnemonic: "ORA",mode: "ABSY",length: 3,cycles: 4,flags: "cZidbvN"},
Code{ code: 0x01,mnemonic: "ORA",mode: "INDX",length: 2,cycles: 6,flags: "cZidbvN"},
Code{ code: 0x11,mnemonic: "ORA",mode: "INDY",length: 2,cycles: 5,flags: "cZidbvN"},
Code{ code: 0x2a,mnemonic: "ROL",mode: "ACC",length: 1,cycles: 2,flags: "CZidbvN"},
Code{ code: 0x26,mnemonic: "ROL",mode: "ZP",length: 2,cycles: 5,flags: "CZidbvN"},
Code{ code: 0x36,mnemonic: "ROL",mode: "ZPX",length: 2,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x2e,mnemonic: "ROL",mode: "ABS",length: 3,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x3e,mnemonic: "ROL",mode: "ABSX",length: 3,cycles: 7,flags: "CZidbvN"},
Code{ code: 0x6a,mnemonic: "ROR",mode: "ACC",length: 1,cycles: 2,flags: "CZidbvN"},
Code{ code: 0x66,mnemonic: "ROR",mode: "ZP",length: 2,cycles: 5,flags: "CZidbvN"},
Code{ code: 0x76,mnemonic: "ROR",mode: "ZPX",length: 2,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x7e,mnemonic: "ROR",mode: "ABS",length: 3,cycles: 6,flags: "CZidbvN"},
Code{ code: 0x6e,mnemonic: "ROR",mode: "ABSX",length: 3,cycles: 7,flags: "CZidbvN"},
Code{ code: 0xe9,mnemonic: "SBC",mode: "IMM",length: 2,cycles: 2,flags: "CZidbVN"},
Code{ code: 0xe5,mnemonic: "SBC",mode: "ZP",length: 2,cycles: 3,flags: "CZidbVN"},
Code{ code: 0xf5,mnemonic: "SBC",mode: "ZPX",length: 2,cycles: 4,flags: "CZidbVN"},
Code{ code: 0xed,mnemonic: "SBC",mode: "ABS",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0xfd,mnemonic: "SBC",mode: "ABSX",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0xf9,mnemonic: "SBC",mode: "ABSY",length: 3,cycles: 4,flags: "CZidbVN"},
Code{ code: 0xe1,mnemonic: "SBC",mode: "INDX",length: 2,cycles: 6,flags: "CZidbVN"},
Code{ code: 0xf1,mnemonic: "SBC",mode: "INDY",length: 2,cycles: 5,flags: "CZidbVN"},
Code{ code: 0x85,mnemonic: "STA",mode: "ZP",length: 2,cycles: 3,flags: "czidbvn"},
Code{ code: 0x95,mnemonic: "STA",mode: "ZPX",length: 2,cycles: 4,flags: "czidbvn"},
Code{ code: 0x8d,mnemonic: "STA",mode: "ABS",length: 3,cycles: 4,flags: "czidbvn"},
Code{ code: 0x9d,mnemonic: "STA",mode: "ABSX",length: 3,cycles: 5,flags: "czidbvn"},
Code{ code: 0x99,mnemonic: "STA",mode: "ABSY",length: 3,cycles: 5,flags: "czidbvn"},
Code{ code: 0x81,mnemonic: "STA",mode: "INDX",length: 2,cycles: 6,flags: "czidbvn"},
Code{ code: 0x91,mnemonic: "STA",mode: "INDY",length: 2,cycles: 6,flags: "czidbvn"},
Code{ code: 0x86,mnemonic: "STX",mode: "ZP",length: 2,cycles: 3,flags: "czidbvn"},
Code{ code: 0x96,mnemonic: "STX",mode: "ZPY",length: 2,cycles: 4,flags: "czidbvn"},
Code{ code: 0x8e,mnemonic: "STX",mode: "ABS",length: 3,cycles: 4,flags: "czidbvn"},
Code{ code: 0x84,mnemonic: "STY",mode: "ZP",length: 2,cycles: 3,flags: "czidbvn"},
Code{ code: 0x94,mnemonic: "STY",mode: "ZPX",length: 2,cycles: 4,flags: "czidbvn"},
Code{ code: 0x8c,mnemonic: "STY",mode: "ABS",length: 3,cycles: 4,flags: "czidbvn"}];


