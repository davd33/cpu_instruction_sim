use std::arch::x86_64::{__cpuid, _rdtsc};
use std::collections::HashMap;
use std::fmt::Display;
use std::{env, fs};
use std::process::exit;

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

#[derive(Eq, PartialEq, Hash)]
enum Command {
    MOV, ADD, SUB, CMP,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Command::MOV => "MOV",
            Command::ADD => "ADD",
            Command::SUB => "SUB",
            Command::CMP => "CMP",
        })
    }
}

#[derive(Eq, Hash, PartialEq)]
enum InstType {
    RegMem,
    ImmediateRegMem,
    ImmediateReg,
    MemAcc,
    AccMem,
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

fn d16_displacement(low: u8, high: u8) -> u16 {
    ((low as u16) & 0x00FF) ^ ((high as u16) << 8)
}

fn d16_signed_displacement(low: u8, high: u8) -> i16 {
    d16_displacement(low, high) as i16
}

fn which_command(byte1: u8, byte2: u8) -> Option<(InstType, String)> {
    let mut mov_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    mov_ids.insert(InstType::RegMem, vec![(0xFC, 0x88)]); // Reg/mem
    mov_ids.insert(InstType::ImmediateRegMem, vec![(0xFE, 0xC6)]); // Immediate reg/mem
    mov_ids.insert(InstType::ImmediateReg, vec![(0xF0, 0xB0)]); // Immediate reg
    mov_ids.insert(InstType::MemAcc, vec![(0xFE, 0xA0)]); // Mem/acc
    mov_ids.insert(InstType::AccMem, vec![(0xFE, 0xA2)]); // Acc/mem

    let mut add_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    add_ids.insert(InstType::RegMem, vec![(0xFC, 0x00)]); // Reg/mem
    add_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x00)]); // Immediate reg/mem
    add_ids.insert(InstType::ImmediateReg, vec![(0xFE, 0x04)]); // Immediate acc

    let mut sub_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    sub_ids.insert(InstType::RegMem, vec![(0xFC, 0x28)]); // Reg/mem
    sub_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x28)]); // Immediate reg/mem
    sub_ids.insert(InstType::ImmediateReg, vec![(0xFE, 0x2C)]); // Immediate acc

    let mut cmp_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    cmp_ids.insert(InstType::RegMem, vec![(0xFC, 0x38)]); // Reg/mem
    cmp_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x38)]); // Immediate reg/mem
    cmp_ids.insert(InstType::ImmediateReg, vec![(0xFE, 0x3C)]); // Immediate acc

    let mut commands: HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>> = HashMap::new();
    commands.insert(Command::MOV, mov_ids);
    commands.insert(Command::ADD, add_ids);
    commands.insert(Command::SUB, sub_ids);
    commands.insert(Command::CMP, cmp_ids);

    for (k, v) in commands {
        for (inst_type, decoder) in v {
            assert!(decoder.len() >= 1 && decoder.len() <= 2, "Instruction decoder must have 1, 2 elements.");
            if let Some((mask, opcode)) = decoder.first() {
                if decoder.len() == 2 {
                    if let Some((mask2, opcode2)) = decoder.get(1) {
                        if ((byte1 & mask) == *opcode) && ((byte2 & mask2) == *opcode2) {
                            return Some((inst_type, k.to_string()));
                        }
                    }
                } else {
                    if (byte1 & mask) == *opcode {
                        return Some((inst_type, k.to_string()));
                    }
                }
            }
        }
    }

    None
}

fn main() {
    let mut cycles_stats = vec![];
    cycles_stats.push(CyclesStat::new("program start", rdtsc()));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name for an ASM to be decoded.");
        exit(1);
    }
    let asm_path = &args[1];

    cycles_stats.push(CyclesStat::new("args read", rdtsc()));

    if let Ok(asm_bytes) = fs::read(asm_path) {
        let mut current = 0;
        let rg_table = mod11_registers_table();
        let rg_mem_table = reg_mem_registers_table();

        // MOD values inplace
        let reg_reg_mod = 0xC0;
        let direct_address_mod = 0x00;
        let d8_mod = 0x40;
        let d16_mod = 0x80;
        let mod11 = 0xC0;

        while current < asm_bytes.len() {
            let (inst_type, command) = match which_command(asm_bytes[current], asm_bytes[current + 1]) {
                None => {
                    print_current_byte(&asm_bytes, current);
                    exit(1);
                }
                Some(cmd) => cmd
            };

            match inst_type {
                InstType::RegMem => {
                    let w_mask = 0x01;
                    let d_mask = 0x02;
                    let mod_mask = 0xC0;
                    let reg_mask = 0x38;
                    let rm_mask = 0x07;

                    let rm = asm_bytes[current + 1] & rm_mask;
                    let reg = (asm_bytes[current + 1] & reg_mask) >> 3;
                    let mod_ = asm_bytes[current + 1] & mod_mask;
                    let d = (asm_bytes[current] & d_mask) >> 1;
                    let w = asm_bytes[current] & w_mask;

                    let reg_str = &rg_table[&(w << 3 | reg)];
                    let rm_str = &rg_table[&(w << 3 | rm)];

                    if mod_ == reg_reg_mod {
                        if d == 1 {
                            println!("{} {}, {}", command, reg_str, rm_str);
                        } else {
                            println!("{} {}, {}", command, rm_str, reg_str);
                        }
                        current += 2;
                    } else if mod_ == direct_address_mod && rm == 0x06 {
                        // DIRECT ADDRESS
                        let disp: u16 = d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

                        println!("{} {}, [{}]", command, reg_str, disp);
                        current += 4;
                    } else if mod_ == d8_mod {
                        let disp: i8 = asm_bytes[current + 2] as i8;

                        let left = format!("{}", reg_str);
                        let right = format!("[{} {} {}]",
                                            rg_mem_table[&((mod_ >> 2) ^ rm)],
                                            if disp < 0 { "" } else { "+" },
                                            disp);

                        if d == 1 {
                            println!("{} {}, {}", command, left, right);
                        } else {
                            println!("{} {}, {}", command, right, left);
                        }
                        current += 3;
                    } else if mod_ == d16_mod {
                        let disp: i16 = d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

                        let left = format!("{}", reg_str);
                        let right = format!("[{} {} {}]",
                                            &rg_mem_table[&(rm)],
                                            if disp < 0 { "" } else { "+" },
                                            disp);

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
                },
                InstType::ImmediateRegMem => {
                    let mod_mask = 0xC0;
                    let w_mask = 0x01;
                    let rm_mask = 0x07;

                    let mod_ = asm_bytes[current + 1] & mod_mask;
                    let w = asm_bytes[current] & w_mask;
                    let rm = asm_bytes[current + 1] & rm_mask;

                    let has_d8 = mod_ == d8_mod;
                    let has_d16 = mod_ == d16_mod;

                    let mut byte_inc = 2;
                    let data_pos = 2 + if has_d8 { 1 } else if has_d16 { 2 } else { 0 };
                    let data = if mod_ == mod11 {
                        byte_inc += 1;
                        format!("{}", asm_bytes[current + data_pos])
                    } else {
                        if w == 0x00 {
                            byte_inc += 1;

                            format!("byte {}", asm_bytes[current + data_pos])
                        } else {
                            byte_inc += 2;

                            format!("word {}", d16_displacement(
                                asm_bytes[current + data_pos],
                                asm_bytes[current + data_pos + 1]))
                        }
                    };

                    let addr: String = if mod_ == d8_mod {
                        byte_inc += 1;
                        let disp: i8 = asm_bytes[current + 2] as i8;
                        format!("[{} {} {}]",
                                rg_mem_table[&((mod_ >> 2) ^ rm)],
                                if disp < 0 { "" } else { "+" },
                                disp)
                    } else if mod_ == d16_mod {
                        byte_inc += 2;
                        let disp: i16 = d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                        format!("[{} {} {}]",
                                &rg_mem_table[&(rm)],
                                if disp < 0 { "" } else { "+" },
                                disp)
                    } else if mod_ == mod11 {
                        format!("{}", rg_table[&((w << 3) ^ rm)])
                    } else {
                        format!("[{}]", rg_mem_table[&(rm)])
                    };

                    println!("{} {}, {} ", command, addr, data);

                    current += byte_inc;
                },
                InstType::ImmediateReg => {
                    let w_mask = 0x08;
                    let reg_mask = 0x07;

                    let w = (asm_bytes[current] & w_mask) >> 3;
                    let reg = asm_bytes[current] & reg_mask;
                    let reg_str = &rg_table[&(w << 3 | reg)];

                    let data: u16 = if w == 0 {
                        asm_bytes[current + 1] as u16
                    } else {
                        d16_displacement(asm_bytes[current + 1], asm_bytes[current + 2])
                    };

                    println!("{} {}, {} ", command, reg_str, data);

                    current += if w == 1 { 3 } else { 2 };
                },
                InstType::MemAcc => {
                    let disp: i16 = d16_signed_displacement(asm_bytes[current + 1], asm_bytes[current + 2]);
                    println!("{} ax, [{}]", command, disp);
                    current += 3;
                },
                InstType::AccMem => {
                    let disp: i16 = d16_signed_displacement(asm_bytes[current + 1], asm_bytes[current + 2]);
                    println!("{} [{}], ax", command, disp);
                    current += 3;
                },
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

fn print_current_byte(asm_bytes: &Vec<u8>, current: usize) {
    println!("No such instruction, ADDR {:02X}/{:02X} = {:02X} ({:08b}) {:02X} ({:08b})",
             current / 16, current % 16,
             asm_bytes[current], asm_bytes[current],
             asm_bytes[current + 1], asm_bytes[current + 1]);
}
