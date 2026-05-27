use std::arch::x86_64::{__cpuid, _rdtsc};
use std::collections::HashMap;
use std::{env, fs};

fn rdtsc() -> u64 {
    unsafe {
        __cpuid(0);
        _rdtsc()
    }
}

struct CyclesStat {
    cycles: u64,
    label: String,
}

impl CyclesStat {
    fn new(label: &str, cycles: u64) -> Self {
        CyclesStat {
            label: label.into(),
            cycles
        }
    }
}

/// The returned table's keys encode a one byte value as follows:
/// 0000 + W (1bit) + REG | R/M (3bits)
/// 16 values in this table: from 0x00 up to 0x0F
///
/// The values are string representations of the register.
fn mod11_registers_table() -> HashMap<u8, String> {
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

/// The returned table's keys encode a one byte value as follows:
/// MOD (1 byte from 0x00 to 0x02)
/// R/M (1 byte from 0x00 to 0x07)
///
/// 3x8 values in this table.
///
/// The values are string representations of registers operations.
fn reg_mem_registers_table() -> HashMap<u8, String> {
    let mut table = HashMap::new();

    // MOD 00
    table.insert(0x00, String::from("BX + SI"));
    table.insert(0x01, String::from("BX + DI"));
    table.insert(0x02, String::from("BP + SI"));
    table.insert(0x03, String::from("BP + DI"));
    table.insert(0x04, String::from("SI"));
    table.insert(0x05, String::from("DI"));
    table.insert(0x06, String::from("DIRECT ADDRESS"));
    table.insert(0x07, String::from("BX"));

    // MOD 01 + D8
    table.insert(0x10, String::from("BX + SI"));
    table.insert(0x11, String::from("BX + DI"));
    table.insert(0x12, String::from("BP + SI"));
    table.insert(0x13, String::from("BP + DI"));
    table.insert(0x14, String::from("SI"));
    table.insert(0x15, String::from("DI"));
    table.insert(0x16, String::from("BP"));
    table.insert(0x17, String::from("BX"));

    // MOD 02 + D16
    table.insert(0x20, String::from("BX + SI"));
    table.insert(0x21, String::from("BX + DI"));
    table.insert(0x22, String::from("BP + SI"));
    table.insert(0x23, String::from("BP + DI"));
    table.insert(0x24, String::from("SI"));
    table.insert(0x25, String::from("DI"));
    table.insert(0x26, String::from("BP"));
    table.insert(0x27, String::from("BX"));

    table
}

fn main() {
    let mut cycles_stats = vec![];
    cycles_stats.push(CyclesStat::new("program start", rdtsc()));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name for an ASM to be decoded.");
        std::process::exit(1);
    }
    let asm_path = &args[1];

    cycles_stats.push(CyclesStat::new("args read", rdtsc()));

    let reg_mem_mask = 0xFC;
    let reg_mem_opcode = 0x88;

    let immediate_to_reg_mask = 0xF0;
    let immediate_to_reg_opcode = 0xB0;

    let immediate_to_reg_mem_mask = 0xFE;
    let immediate_to_reg_mem_opcode = 0xC6;

    if let Ok(asm_bytes) = fs::read(asm_path) {
        let mut current = 0;
        let rg_table = mod11_registers_table();
        let rg_mem_table = reg_mem_registers_table();
        let command = "MOV";

        while current < asm_bytes.len() {
            if asm_bytes[current] & reg_mem_mask == reg_mem_opcode {
                let w_mask = 0x01;
                let d_mask = 0x02;
                let mod_mask = 0xC0;
                let reg_mask = 0x38;
                let rm_mask = 0x07;
                let reg_reg_mod = 0xC0;
                let direct_address_mod = 0x00;
                let d8_mod = 0x40;
                let d16_mod = 0x80;

                let rm = asm_bytes[current + 1] & rm_mask;
                let reg = (asm_bytes[current + 1] & reg_mask) >> 3;
                let mod_ = asm_bytes[current + 1] & mod_mask;
                let d = (asm_bytes[current] & d_mask) >> 1;
                let w = asm_bytes[current] & w_mask;

                let reg_str = &rg_table[&(w << 3 | reg)];
                let rm_str = &rg_table[&(w << 3 | rm)];

                if mod_ == reg_reg_mod {
                    // register to register move
                    if d == 1 {
                        println!("{} {}, {}", command, reg_str, rm_str);
                    } else {
                        println!("{} {}, {}", command, rm_str, reg_str);
                    }
                    current += 2;
                } else if mod_ == direct_address_mod && rm == 0x06 {
                    // direct address NOT IMPLEMENTED
                    let low: u16 = asm_bytes[current + 2] as u16 & 0x00FF;
                    let high: u16 = (asm_bytes[current + 3] as u16) << 8;
                    let t = &rg_table[&((low ^ high) as u8)];
                    println!("{} {}, {}", command, reg_str, t);
                    current += 4;
                } else if mod_ == d8_mod {
                    // 8 bits displacement
                    let low: i8 = asm_bytes[current + 2] as i8;

                    let left = format!("{}", reg_str);
                    let right = format!("[{} {} {}]",
                                        rg_mem_table[&((mod_ >> 2) ^ rm)],
                                        if low < 0 { "" } else { "+" },
                                        low);

                    if d == 1 {
                        println!("{} {}, {}", command, left, right);
                    } else {
                        println!("{} {}, {}", command, right, left);
                    }
                    current += 3;
                } else if mod_ == d16_mod {
                    // 16 bits displacement
                    let low: u16 = asm_bytes[current + 2] as u16 & 0x00FF;
                    let high: u16 = (asm_bytes[current + 3] as u16) << 8;
                    let displacement: i16 = (low ^ high) as i16;

                    let left = format!("{}", reg_str);
                    let right = format!("[{} {} {}]",
                                        &rg_mem_table[&(rm)],
                                        if displacement < 0 { "" } else { "+" },
                                        displacement);

                    if d == 1 {
                        println!("{} {}, {}", command, left, right);
                    } else {
                        println!("{} {}, {}", command, right, left);
                    }
                    current += 4;
                } else {
                    // no displacement
                    let left = format!("{}", reg_str);
                    let right = format!("[{}]", rg_mem_table[&(rm)]);

                    if d == 1 {
                        println!("{} {}, {}", command, left, right);
                    } else {
                        println!("{} {}, {}", command, right, left);
                    }

                    current += 2;
                }
            } else if asm_bytes[current] & immediate_to_reg_mask == immediate_to_reg_opcode {
                let w_mask = 0x08;
                let reg_mask = 0x07;

                let w = (asm_bytes[current] & w_mask) >> 3;
                let reg = asm_bytes[current] & reg_mask;
                let reg_str = &rg_table[&(w << 3 | reg)];

                let data: u16 = if w == 0 {
                    asm_bytes[current + 1] as u16
                } else {
                    let low: u16 = (asm_bytes[current + 1] as u16) & 0x00FF;
                    let high: u16 = (asm_bytes[current + 2] as u16) << 8;

                    low ^ high
                };

                println!("{} {}, {} ", command, reg_str, data);

                current += if w == 1 { 3 } else { 2 };
            } else if asm_bytes[current] & immediate_to_reg_mem_mask == immediate_to_reg_mem_opcode {
                let mod_mask = 0xC0;
                let w_mask = 0x01;
                let rm_mask = 0x07;

                let mod_ = asm_bytes[current + 1] & mod_mask >> 6;
                let w = asm_bytes[current] & w_mask;
                let rm = asm_bytes[current + 1] & rm_mask;

                let data = if w == 0x01 {
                    asm_bytes[current + 2] as u16
                } else {
                    asm_bytes[current + 3] as u16
                };

                println!("{} {}, {} ", command, mod_, rm);

                current += 3;
                if w == 1 {
                    current += 1;
                }

                // if displacement inc current too
            }
        }
    } else {
        println!("File not found");
    }

    cycles_stats.push(CyclesStat::new("inst. stream printed", rdtsc()));
    let mut current_cycles = cycles_stats[0].cycles;
    for c in &cycles_stats {
        eprintln!("{} {}", c.label, c.cycles - current_cycles);
        current_cycles = c.cycles;
    }
}
