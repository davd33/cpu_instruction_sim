use std::{env, fs};
use std::collections::HashMap;

fn build_registers_table() -> HashMap<u8, String> {
    let mut table = HashMap::new();

    // W = 0
    table.insert(0x00, String::from("AL"));
    table.insert(0x01, String::from("CL"));
    table.insert(0x02, String::from("DL"));
    table.insert(0x03, String::from("BL"));
    table.insert(0x04, String::from("AH"));
    table.insert(0x05, String::from("CH"));
    table.insert(0x06, String::from("DH"));
    table.insert(0x07, String::from("BH"));

    // W = 1
    table.insert(0x08, String::from("AX"));
    table.insert(0x09, String::from("CX"));
    table.insert(0x0A, String::from("DX"));
    table.insert(0x0B, String::from("BX"));
    table.insert(0x0C, String::from("SP"));
    table.insert(0x0D, String::from("BP"));
    table.insert(0x0E, String::from("SI"));
    table.insert(0x0F, String::from("DI"));
    table
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name for an ASM to be decoded.");
        std::process::exit(1);
    }
    let asm_path = &args[1];

    let op_code_mask = 0xFC;
    let d_mask = 0x02;
    let w_mask = 0x01;
    let mod_mask = 0xC0;
    let reg_mask = 0x38;
    let rm_mask = 0x07;

    let mov_op_code = 0x88;
    let mov_reg_mod = 0xC0;

    let rg_table = build_registers_table();
    if let Ok(asm_bytes) = fs::read(asm_path) {
        let mut current = 0;
        while current < asm_bytes.len() {
            if asm_bytes[current] & op_code_mask == mov_op_code {
                print!("MOV");
                if asm_bytes[current + 1] & mod_mask == mov_reg_mod {
                    let d = (asm_bytes[current] & d_mask) >> 1;
                    let w = asm_bytes[current] & w_mask;
                    let reg = (asm_bytes[current + 1] & reg_mask) >> 3;
                    let rm = asm_bytes[current + 1] & rm_mask;

                    let reg_str = &rg_table[&(w << 3 | reg)];
                    let rm_str = &rg_table[&(w << 3 | rm)];
                    if d == 1 {
                        print!(" {}, {}", reg_str, rm_str);
                    } else {
                        print!(" {}, {}", rm_str, reg_str);
                    }
                    println!();
                    current += 2;
                }
            }
        }
    } else {
        println!("File not found");
    }
}
