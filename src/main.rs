mod cpu_sim;
mod decode;
mod cycles_stats;

use std::fmt::{Display, Formatter};
use std::{env, fs};
use std::process::exit;
use cpu_sim::{CommandImpl, Mov, CPU8086};
use cycles_stats::CyclesStat;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Command {
    MOV, ADD, SUB, CMP,
    JNZ, JE, JL, JLE, JB,
    JBE, JP, JO, JS, JNE, JNL,
    JG, JNB, JA, JNP, JNO, JNS,
    LOOP, LOOPZ, LOOPNZ, JCXZ
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Command::MOV => "MOV",
            Command::ADD => "ADD",
            Command::SUB => "SUB",
            Command::CMP => "CMP",
            Command::JNZ => "JNZ",
            Command::JE => "JE",
            Command::JL => "JL",
            Command::JLE => "JLE",
            Command::JB => "JB",
            Command::JBE => "JBE",
            Command::JP => "JP",
            Command::JO => "JO",
            Command::JS => "JS",
            Command::JNE => "JNE",
            Command::JNL => "JNL",
            Command::JG => "JG",
            Command::JNB => "JNB",
            Command::JA => "JA",
            Command::JNP => "JNP",
            Command::JNO => "JNO",
            Command::JNS => "JNS",
            Command::LOOP => "LOOP",
            Command::LOOPZ => "LOOPZ",
            Command::LOOPNZ => "LOOPNZ",
            Command::JCXZ => "JCXZ",
        })
    }
}


trait Operand {}
struct RegisterOp {
    register: Register,
}
impl Operand for RegisterOp {}
struct ImmediateOp {
    value: u16,
}
impl Operand for ImmediateOp {}

impl Into<Box<dyn CommandImpl>> for Instruction<RegisterOp, ImmediateOp> {
    fn into(self) -> Box<dyn CommandImpl> {
        match self.command {
            Command::MOV => Box::new(Mov::<RegisterOp, ImmediateOp> {
                instruction: self
            }),
            _ => unimplemented!()
        }
    }
}

struct Instruction<T: Operand, E: Operand> {
    command: Command,
    op1: T,
    op2: E,
}

#[derive(Eq, Hash, PartialEq)]
enum InstType {
    RegMem,
    ImmediateRegMem,
    ImmediateReg,
    MemAcc,
    AccMem,
    ImmediateToAcc,
    ToLabel,
}

#[derive(Clone, Copy)]
enum Register {
    AX, AL, AH,
    BX, BL, BH,
    CX, CL, CH,
    DX, DL, DH,
    BP,
    SP,
    DI,
    SI,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let reg_str = match self {
            Register::AX => "AX",
            Register::AL => "AL",
            Register::AH => "AH",
            Register::BX => "BX",
            Register::BL => "BL",
            Register::BH => "BH",
            Register::CX => "CX",
            Register::CL => "CL",
            Register::CH => "CH",
            Register::DX => "DX",
            Register::DL => "DL",
            Register::DH => "DH",
            Register::BP => "BP",
            Register::SP => "SP",
            Register::DI => "DI",
            Register::SI => "SI",
        };

        write!(f, "{}", reg_str)
    }
}

fn main() {
    let mut cycles_stats = vec![];
    cycles_stats.push(CyclesStat::new("program start", cycles_stats::rdtsc()));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name for an ASM to be decoded.");
        exit(1);
    }
    let asm_path = &args[1];

    cycles_stats.push(CyclesStat::new("args read", cycles_stats::rdtsc()));

    if let Ok(asm_bytes) = fs::read(asm_path) {
        let mut current = 0;
        let rg_table = decode::mod11_registers_table();
        let rg_mem_table = decode::reg_mem_registers_table();

        // MOD values inplace
        let reg_reg_mod = 0xC0;
        let direct_address_mod = 0x00;
        let d8_mod = 0x40;
        let d16_mod = 0x80;
        let mod11 = 0xC0;

        let commands = decode::get_commands_map();
        let mut cpu = CPU8086::new();

        while current < asm_bytes.len() {
            let (inst_type, command) = match decode::which_command(asm_bytes[current], asm_bytes[current + 1], &commands) {
                None => {
                    println!("No such instruction. {}", decode::format_current_byte(&asm_bytes, current));
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
                        let disp: u16 = decode::d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

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
                        let disp: i16 = decode::d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);

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
                    let s_mask = 0x02;
                    let rm_mask = 0x07;

                    let mod_ = asm_bytes[current + 1] & mod_mask;
                    let s = (asm_bytes[current] & s_mask) >> 1;
                    let w = asm_bytes[current] & w_mask;
                    let rm = asm_bytes[current + 1] & rm_mask;

                    let has_d8 = mod_ == d8_mod;
                    let has_d16 = mod_ == d16_mod || (mod_ == direct_address_mod && rm == 6);

                    let mut byte_inc = 2;
                    let data_pos = 2 + if has_d8 { 1 } else if has_d16 { 2 } else { 0 };
                    let data = if mod_ == mod11 {
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
                                format!("{}{}", word, decode::d8_signed_extended(asm_bytes[current + data_pos]))
                            } else {
                                byte_inc += 2;
                                format!("{}{}", word, decode::d16_displacement(
                                    asm_bytes[current + data_pos],
                                    asm_bytes[current + data_pos + 1]))
                            }
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
                        let disp: i16 = decode::d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                        format!("[{} {} {}]",
                                &rg_mem_table[&(rm)],
                                if disp < 0 { "" } else { "+" },
                                disp)
                    } else if mod_ == direct_address_mod && rm == 0x06 {
                        byte_inc += 2;
                        let disp: u16 = decode::d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                        format!("[{}]", disp)
                    } else if mod_ == mod11 {
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
                        decode::d16_displacement(asm_bytes[current + 1], asm_bytes[current + 2])
                    };

                    println!("{} {}, {} ", command, register, data);
                    let mut mov_cmd: Box<dyn CommandImpl> = Instruction {
                        command: command.clone(),
                        op1: RegisterOp {register: register.clone()},
                        op2: ImmediateOp {value: data},
                    }.into();
                    mov_cmd.execute(&mut cpu);

                    current += if w == 1 { 3 } else { 2 };
                },
                InstType::MemAcc => {
                    let disp: i16 = decode::d16_signed_displacement(asm_bytes[current + 1], asm_bytes[current + 2]);
                    println!("{} ax, [{}]", command, disp);
                    current += 3;
                },
                InstType::AccMem => {
                    let disp: i16 = decode::d16_signed_displacement(asm_bytes[current + 1], asm_bytes[current + 2]);
                    println!("{} [{}], ax", command, disp);
                    current += 3;
                },
                InstType::ImmediateToAcc => {
                    let w_mask = 0x01;

                    let w = asm_bytes[current] & w_mask;

                    let data: u16 = if w == 0 {
                        asm_bytes[current + 1] as u16
                    } else {
                        decode::d16_displacement(asm_bytes[current + 1], asm_bytes[current + 2])
                    };

                    let reg = if w == 0 { "al" } else { "ax" };

                    println!("{} {}, {} ", command, reg, data);

                    current += if w == 1 { 3 } else { 2 };
                },
                InstType::ToLabel => {
                    let jump = asm_bytes[current + 1] as i8;
                    println!("{} ${:+}", command, jump+2);

                    current += 2;
                },
            }
        }

        println!("RESULT = {}", cpu)
    } else {
        println!("File not found");
    }

    cycles_stats.push(CyclesStat::new("inst. stream printed", cycles_stats::rdtsc()));
    let mut current_cycles = cycles_stats[0].cycles;
    for c in &cycles_stats {
        eprintln!("{} {}", c.label, c.cycles - current_cycles);
        current_cycles = c.cycles;
    }
}

