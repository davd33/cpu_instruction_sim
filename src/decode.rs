use std::collections::HashMap;
use std::process::exit;
use crate::cpu_sim::{Command, CommandImpl, ImmediateOp, Instruction, Register, RegisterOp, CPU8086};
use crate::{AppMode, D8_MOD, D16_MOD, DIRECT_ADDRESS_MOD, MOD11};

/// The returned table's keys encode a one byte value as follows:
/// 0000 + W (1bit) + REG | R/M (3bits)
/// 16 values in this table: from 0x00 up to 0x0F
///
/// The values are string representations of the register.
pub fn mod11_registers_table() -> HashMap<u8, Register> {
    let mut table: HashMap<u8, Register> = HashMap::new();

    // W = 0
    table.insert(0x00, Register::AL);
    table.insert(0x01, Register::CL);
    table.insert(0x02, Register::DL);
    table.insert(0x03, Register::BL);
    table.insert(0x04, Register::AH);
    table.insert(0x05, Register::CH);
    table.insert(0x06, Register::DH);
    table.insert(0x07, Register::BH);

    // W = 1
    table.insert(0x08, Register::AX);
    table.insert(0x09, Register::CX);
    table.insert(0x0A, Register::DX);
    table.insert(0x0B, Register::BX);
    table.insert(0x0C, Register::SP);
    table.insert(0x0D, Register::BP);
    table.insert(0x0E, Register::SI);
    table.insert(0x0F, Register::DI);
    table
}

/// The returned table's keys encode a one byte value as follows:
/// MOD (1 byte from 0x00 to 0x02)
/// R/M (1 byte from 0x00 to 0x07)
///
/// 3x8 values in this table.
///
/// The values are string representations of registers operations.
pub fn reg_mem_registers_table() -> HashMap<u8, String> {
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

pub fn d16_displacement(low: u8, high: u8) -> u16 {
    ((low as u16) & 0x00FF) ^ ((high as u16) << 8)
}

pub fn d16_signed_displacement(low: u8, high: u8) -> i16 {
    d16_displacement(low, high) as i16
}

pub fn d8_signed_extended(low: u8) -> i16 {
    low as i16
}

pub fn get_commands_map() -> HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>> {
    let mut mov_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    mov_ids.insert(InstType::RegMem, vec![(0xFC, 0x88)]); // Reg/mem
    mov_ids.insert(InstType::ImmediateRegMem, vec![(0xFE, 0xC6)]); // Immediate reg/mem
    mov_ids.insert(InstType::ImmediateReg, vec![(0xF0, 0xB0)]); // Immediate reg
    mov_ids.insert(InstType::MemAcc, vec![(0xFE, 0xA0)]); // Mem/acc
    mov_ids.insert(InstType::AccMem, vec![(0xFE, 0xA2)]); // Acc/mem

    let mut add_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    add_ids.insert(InstType::RegMem, vec![(0xFC, 0x00)]); // Reg/mem
    add_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x00)]); // Immediate reg/mem
    add_ids.insert(InstType::ImmediateToAcc, vec![(0xFE, 0x04)]); // Immediate acc

    let mut sub_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    sub_ids.insert(InstType::RegMem, vec![(0xFC, 0x28)]); // Reg/mem
    sub_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x28)]); // Immediate reg/mem
    sub_ids.insert(InstType::ImmediateToAcc, vec![(0xFE, 0x2C)]); // Immediate acc

    let mut cmp_ids: HashMap<InstType, Vec<(u8, u8)>> = HashMap::new();
    cmp_ids.insert(InstType::RegMem, vec![(0xFC, 0x38)]); // Reg/mem
    cmp_ids.insert(InstType::ImmediateRegMem, vec![(0xFC, 0x80), (0x38, 0x38)]); // Immediate reg/mem
    cmp_ids.insert(InstType::ImmediateToAcc, vec![(0xFE, 0x3C)]); // Immediate acc

    let mut commands: HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>> = HashMap::new();
    commands.insert(Command::MOV, mov_ids);
    commands.insert(Command::ADD, add_ids);
    commands.insert(Command::SUB, sub_ids);
    commands.insert(Command::CMP, cmp_ids);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x75)]);
    commands.insert(Command::JNZ, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x74)]);
    commands.insert(Command::JE, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7C)]);
    commands.insert(Command::JL, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7E)]);
    commands.insert(Command::JLE, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x72)]);
    commands.insert(Command::JB, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x76)]);
    commands.insert(Command::JBE, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7A)]);
    commands.insert(Command::JP, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x70)]);
    commands.insert(Command::JO, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x78)]);
    commands.insert(Command::JS, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x75)]);
    commands.insert(Command::JNE, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7D)]);
    commands.insert(Command::JNL, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7F)]);
    commands.insert(Command::JG, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x73)]);
    commands.insert(Command::JNB, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x77)]);
    commands.insert(Command::JA, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x7B)]);
    commands.insert(Command::JNP, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x71)]);
    commands.insert(Command::JNO, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0x79)]);
    commands.insert(Command::JNS, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0xE2)]);
    commands.insert(Command::LOOP, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0xE1)]);
    commands.insert(Command::LOOPZ, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0xE0)]);
    commands.insert(Command::LOOPNZ, insts);
    let mut insts = HashMap::new();
    insts.insert(InstType::ToLabel, vec![(0xFF, 0xE3)]);
    commands.insert(Command::JCXZ, insts);

    commands
}

pub fn which_command(byte1: u8, byte2: u8, commands: &HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>>) -> Option<(&InstType, &Command)> {
    for (k, v) in commands {
        for (inst_type, decoder) in v {
            assert!(!decoder.is_empty() && decoder.len() <= 2, "Instruction decoder must have 1, 2 elements.");
            if let Some((mask, opcode)) = decoder.first() {
                if decoder.len() == 2 {
                    if let Some((mask2, opcode2)) = decoder.get(1) && ((byte1 & mask) == *opcode) && ((byte2 & mask2) == *opcode2) {
                        return Some((inst_type, k));
                    }
                } else {
                    if (byte1 & mask) == *opcode {
                        return Some((inst_type, k));
                    }
                }
            }
        }
    }

    None
}

pub fn format_current_byte(asm_bytes: &Vec<u8>, current: usize) -> String {
    format!("ADDR {:02X}/{:02X} = {:02X} ({:08b}) {:02X} ({:08b})",
             current / 16, current % 16,
             asm_bytes[current], asm_bytes[current],
             asm_bytes[current + 1], asm_bytes[current + 1])
}

#[derive(Eq, Hash, PartialEq)]
pub enum InstType {
    RegMem,
    ImmediateRegMem,
    ImmediateReg,
    MemAcc,
    AccMem,
    ImmediateToAcc,
    ToLabel,
}

pub fn process_instruction(
    app_mode: &AppMode,
    debug: bool,
    asm_bytes: &Vec<u8>,
    current_byte: usize,
    cpu: &mut Option<CPU8086>,
    rg_table: &HashMap<u8, Register>,
    rg_mem_table: &HashMap<u8, String>,
    commands: &HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>>) -> usize {
    
    let mut current: usize = current_byte;
    
    let (inst_type, command) = match which_command(asm_bytes[current], asm_bytes[current + 1], commands) {
        None => {
            println!("No such instruction. {}", format_current_byte(asm_bytes, current));
            exit(1);
        }
        Some(cmd) => cmd
    };

    if debug {
        println!("{}", format_current_byte(asm_bytes, current));
    }

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

            let reg = &rg_table[&(w << 3 | reg)];
            let reg_m = &rg_table[&(w << 3 | rm)];

            if mod_ == MOD11 {
                if d == 1 {
                    println!("{} {}, {}", command, reg, reg_m);

                    if app_mode == &AppMode::Simulation {
                        let mut mov_cmd: Box<dyn CommandImpl> = Instruction {
                            command: command.clone(),
                            op1: RegisterOp { register: *reg_m },
                            op2: RegisterOp { register: *reg },
                        }.into();
                        cpu_exec(cpu, &mut mov_cmd);
                    }
                } else {
                    println!("{} {}, {}", command, reg_m, reg);

                    if app_mode == &AppMode::Simulation {
                        let mut mov_cmd: Box<dyn CommandImpl> = Instruction {
                            command: command.clone(),
                            op1: RegisterOp { register: *reg },
                            op2: RegisterOp { register: *reg_m },
                        }.into();
                        cpu_exec(cpu, &mut mov_cmd);
                    }
                }
                current += 2;
            } else if mod_ == DIRECT_ADDRESS_MOD && rm == 0x06 {
                // DIRECT ADDRESS
                let disp: u16 = d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

                println!("{} {}, [{}]", command, reg, disp);
                current += 4;
            } else if mod_ == D8_MOD {
                let disp: i8 = asm_bytes[current + 2] as i8;

                let left = format!("{}", reg);
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
            } else if mod_ == D16_MOD {
                let disp: i16 = d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

                let left = format!("{}", reg);
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
                let left = format!("{}", reg);
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
            let s_mask = 0x02;
            let rm_mask = 0x07;

            let mod_ = asm_bytes[current + 1] & mod_mask;
            let s = (asm_bytes[current] & s_mask) >> 1;
            let w = asm_bytes[current] & w_mask;
            let rm = asm_bytes[current + 1] & rm_mask;

            let has_d8 = mod_ == D8_MOD;
            let has_d16 = mod_ == D16_MOD || (mod_ == DIRECT_ADDRESS_MOD && rm == 6);

            let mut byte_inc = 2;
            let data_pos = 2 + if has_d8 { 1 } else if has_d16 { 2 } else { 0 };
            let data = if mod_ == MOD11 && *command == Command::MOV {
                byte_inc += 1;
                format!("{}", asm_bytes[current + data_pos])
            } else {
                if w == 0x00 {
                    byte_inc += 1;

                    let byte = if *command == Command::MOV { "byte " } else { "" };
                    format!("{}{}", byte, asm_bytes[current + data_pos])
                } else {
                    let word = if *command == Command::MOV { "word " } else { "" };
                    if s == 1 && *command != Command::MOV {
                        byte_inc += 1;
                        format!("{}{}", word, d8_signed_extended(asm_bytes[current + data_pos]))
                    } else {
                        byte_inc += 2;
                        format!("{}{}", word, d16_displacement(
                            asm_bytes[current + data_pos],
                            asm_bytes[current + data_pos + 1]))
                    }
                }
            };

            let addr: String = if mod_ == D8_MOD {
                byte_inc += 1;
                let disp: i8 = asm_bytes[current + 2] as i8;
                format!("[{} {} {}]",
                        rg_mem_table[&((mod_ >> 2) ^ rm)],
                        if disp < 0 { "" } else { "+" },
                        disp)
            } else if mod_ == D16_MOD {
                byte_inc += 2;
                let disp: i16 = d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                format!("[{} {} {}]",
                        &rg_mem_table[&(rm)],
                        if disp < 0 { "" } else { "+" },
                        disp)
            } else if mod_ == DIRECT_ADDRESS_MOD && rm == 0x06 {
                byte_inc += 2;
                let disp: u16 = d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                format!("[{}]", disp)
            } else if mod_ == MOD11 {
                format!("{}", rg_table[&((w << 3) ^ rm)])
            } else {
                format!("[{}]", rg_mem_table[&(rm)])
            };

            let addr_size = if *command != Command::MOV && addr.contains("[") {
                if w == 0 {
                    "byte "
                } else {
                    "word "
                }
            } else {
                ""
            };

            println!("{} {}{}, {} ", command, addr_size, addr, data);

            current += byte_inc;
        },
        InstType::ImmediateReg => {
            let w_mask = 0x08;
            let reg_mask = 0x07;

            let w = (asm_bytes[current] & w_mask) >> 3;
            let reg = asm_bytes[current] & reg_mask;
            let register = &rg_table[&(w << 3 | reg)];

            let data: u16 = if w == 0 {
                asm_bytes[current + 1] as u16
            } else {
                d16_displacement(asm_bytes[current + 1], asm_bytes[current + 2])
            };

            println!("{} {}, {}", command, register, data);

            if app_mode == &AppMode::Simulation {
                let mut mov_cmd: Box<dyn CommandImpl> = Instruction {
                    command: command.clone(),
                    op1: RegisterOp { register: *register },
                    op2: ImmediateOp { value: data },
                }.into();
                cpu_exec(cpu, &mut mov_cmd);
            }

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
        InstType::ImmediateToAcc => {
            let w_mask = 0x01;

            let w = asm_bytes[current] & w_mask;

            let data: u16 = if w == 0 {
                asm_bytes[current + 1] as u16
            } else {
                d16_displacement(asm_bytes[current + 1], asm_bytes[current + 2])
            };

            let reg = if w == 0 { "al" } else { "ax" };

            println!("{} {}, {} ", command, reg, data);

            current += if w == 1 { 3 } else { 2 };
        },
        InstType::ToLabel => {
            let jump = asm_bytes[current + 1] as i8;
            println!("{} ${:+}", command, jump + 2);

            current += 2;
        },
    }
    
    current
}

fn cpu_exec(cpu: &mut Option<CPU8086>, cmd: &mut Box<dyn CommandImpl>) {
    if let Some(cpu) = cpu {
        cmd.execute(cpu);
    }
}
