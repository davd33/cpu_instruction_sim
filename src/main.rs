use std::arch::x86_64::{__cpuid, _rdtsc};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

struct CPU8086 {
    regs: [u16; 8],
    ax: usize,
    bx: usize,
    cx: usize,
    dx: usize,
    bp: usize,
    sp: usize,
    di: usize,
    si: usize,
}

impl CPU8086 {
    fn new() -> Self {
        CPU8086 {
            regs: [0,0,0,0,0,0,0,0],
            ax: 0,
            bx: 1,
            cx: 2,
            dx: 3,
            bp: 4,
            sp: 5,
            di: 6,
            si: 7,
        }
    }

    fn get(&self, reg: Register) -> u16 {
        match reg {
            Register::AX => self.ax(),
            Register::AL => self.al() as u16,
            Register::AH => self.ah() as u16,
            Register::BX => self.bx(),
            Register::BL => self.bl() as u16,
            Register::BH => self.bh() as u16,
            Register::CX => self.cx(),
            Register::CL => self.cl() as u16,
            Register::CH => self.ch() as u16,
            Register::DX => self.dx(),
            Register::DL => self.dl() as u16,
            Register::DH => self.dh() as u16,
            Register::BP => self.bp(),
            Register::SP => self.sp(),
            Register::DI => self.di(),
            Register::SI => self.si(),
        }
    }

    fn set(&mut self, reg: Register, val: u16) {
        match reg {
            Register::AX => self.set_ax(val),
            Register::AL => self.set_al(val as u8),
            Register::AH => self.set_ah(val as u8),
            Register::BX => self.set_bx(val),
            Register::BL => self.set_bl(val as u8),
            Register::BH => self.set_bh(val as u8),
            Register::CX => self.set_cx(val),
            Register::CL => self.set_cl(val as u8),
            Register::CH => self.set_ch(val as u8),
            Register::DX => self.set_dx(val),
            Register::DL => self.set_dl(val as u8),
            Register::DH => self.set_dh(val as u8),
            Register::BP => self.set_bp(val),
            Register::SP => self.set_sp(val),
            Register::DI => self.set_di(val),
            Register::SI => self.set_si(val),
        }
    }

    fn set_bp(&mut self, val: u16) {
        self.regs[self.bp] = val;
    }

    fn bp(&self) -> u16 {
        self.regs[self.bp]
    }

    fn set_sp(&mut self, val: u16) {
        self.regs[self.sp] = val;
    }

    fn sp(&self) -> u16 {
        self.regs[self.sp]
    }

    fn set_di(&mut self, val: u16) {
        self.regs[self.di] = val;
    }

    fn di(&self) -> u16 {
        self.regs[self.di]
    }

    fn set_si(&mut self, val: u16) {
        self.regs[self.si] = val;
    }

    fn si(&self) -> u16 {
        self.regs[self.si]
    }

    // A

    fn set_ax(&mut self, val: u16) {
        self.regs[self.ax] = val;
    }

    fn ax(&self) -> u16 {
        self.al() as u16 ^ ((self.ah() as u16) << 8)
    }

    fn set_al(&mut self, val: u8) {
        self.regs[self.ax] = ((val as u16) << 8) ^ self.ax();
    }

    fn al(&self) -> u8 {
        (self.regs[self.ax] >> 8) as u8
    }

    fn set_ah(&mut self, val: u8) {
        self.regs[self.ax] = ((self.al() as u16) << 8) ^ val as u16;
    }

    fn ah(&self) -> u8 {
        self.regs[self.ax] as u8
    }

    // B

    fn set_bx(&mut self, val: u16) {
        self.regs[self.bx] = val;
    }

    fn bx(&self) -> u16 {
        self.bl() as u16 ^ ((self.bh() as u16) << 8)
    }

    fn set_bl(&mut self, val: u8) {
        self.regs[self.bx] = ((val as u16) << 8) ^ self.bx();
    }

    fn bl(&self) -> u8 {
        (self.regs[self.bx] >> 8) as u8
    }

    fn set_bh(&mut self, val: u8) {
        self.regs[self.bx] = ((self.bl() as u16) << 8) ^ val as u16;
    }

    fn bh(&self) -> u8 {
        self.regs[self.bx] as u8
    }

    // C

    fn set_cx(&mut self, val: u16) {
        self.regs[self.cx] = val;
    }

    fn cx(&self) -> u16 {
        self.cl() as u16 ^ ((self.ch() as u16) << 8)
    }

    fn set_cl(&mut self, val: u8) {
        self.regs[self.cx] = ((val as u16) << 8) ^ self.cx();
    }

    fn cl(&self) -> u8 {
        (self.regs[self.cx] >> 8) as u8
    }

    fn set_ch(&mut self, val: u8) {
        self.regs[self.cx] = ((self.cl() as u16) << 8) ^ val as u16;
    }

    fn ch(&self) -> u8 {
        self.regs[self.cx] as u8
    }

    // D

    fn set_dx(&mut self, val: u16) {
        self.regs[self.dx] = val;
    }

    fn dx(&self) -> u16 {
        self.dl() as u16 ^ ((self.dh() as u16) << 8)
    }

    fn set_dl(&mut self, val: u8) {
        self.regs[self.dx] = ((val as u16) << 8) ^ self.dx();
    }

    fn dl(&self) -> u8 {
        (self.regs[self.dx] >> 8) as u8
    }

    fn set_dh(&mut self, val: u8) {
        self.regs[self.dx] = ((self.dl() as u16) << 8) ^ val as u16;
    }

    fn dh(&self) -> u8 {
        self.regs[self.dx] as u8
    }
}

#[derive(Eq, PartialEq, Hash)]
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

/// The returned table's keys encode a one byte value as follows:
/// 0000 + W (1bit) + REG | R/M (3bits)
/// 16 values in this table: from 0x00 up to 0x0F
///
/// The values are string representations of the register.
fn mod11_registers_table() -> HashMap<u8, Register> {
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

fn d8_signed_extended(low: u8) -> i16 {
    low as i16
}

fn get_commands_map() -> HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>> {
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

fn which_command(byte1: u8, byte2: u8, commands: &HashMap<Command, HashMap<InstType, Vec<(u8, u8)>>>) -> Option<(&InstType, &Command)> {
    for (k, v) in commands {
        for (inst_type, decoder) in v {
            assert!(decoder.len() >= 1 && decoder.len() <= 2, "Instruction decoder must have 1, 2 elements.");
            if let Some((mask, opcode)) = decoder.first() {
                if decoder.len() == 2 {
                    if let Some((mask2, opcode2)) = decoder.get(1) {
                        if ((byte1 & mask) == *opcode) && ((byte2 & mask2) == *opcode2) {
                            return Some((inst_type, k));
                        }
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

        let commands = get_commands_map();

        while current < asm_bytes.len() {
            let (inst_type, command) = match which_command(asm_bytes[current], asm_bytes[current + 1], &commands) {
                None => {
                    println!("No such instruction. {}", format_current_byte(&asm_bytes, current));
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
                                format!("{}{}", word, d8_signed_extended(asm_bytes[current + data_pos]))
                            } else {
                                byte_inc += 2;
                                format!("{}{}", word, d16_displacement(
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
                        let disp: i16 = d16_signed_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
                        format!("[{} {} {}]",
                                &rg_mem_table[&(rm)],
                                if disp < 0 { "" } else { "+" },
                                disp)
                    } else if mod_ == direct_address_mod && rm == 0x06 {
                        byte_inc += 2;
                        let disp: u16 = d16_displacement(asm_bytes[current + 2], asm_bytes[current + 3]);
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
                    println!("{} ${:+}", command, jump+2);

                    current += 2;
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

fn format_current_byte(asm_bytes: &Vec<u8>, current: usize) -> String {
    format!("ADDR {:02X}/{:02X} = {:02X} ({:08b}) {:02X} ({:08b})",
             current / 16, current % 16,
             asm_bytes[current], asm_bytes[current],
             asm_bytes[current + 1], asm_bytes[current + 1])
}

#[cfg(test)]
mod tests {
    use crate::CPU8086;

    #[test]
    fn test_cpu_register_access() {
        // A
        let mut cpu = CPU8086::new();
        cpu.set_al(1);
        assert_eq!(cpu.ax(), 1);
        cpu.set_al(0);
        cpu.set_ah(1);
        assert_eq!(cpu.ax(), 256);
        // B
        let mut cpu = CPU8086::new();
        cpu.set_bl(0x02);
        assert_eq!(cpu.bx(), 2);
        cpu.set_bl(0);
        cpu.set_bh(0x02);
        assert_eq!(cpu.bx(), 512);
        // C
        let mut cpu = CPU8086::new();
        cpu.set_cl(0x04);
        assert_eq!(cpu.cx(), 4);
        cpu.set_cl(0);
        cpu.set_ch(0x04);
        assert_eq!(cpu.cx(), 1024);
        // D
        let mut cpu = CPU8086::new();
        cpu.set_dl(0x08);
        assert_eq!(cpu.dx(), 8);
        cpu.set_dl(0);
        cpu.set_dh(0x08);
        assert_eq!(cpu.dx(), 2048);
        // BP
        let mut cpu = CPU8086::new();
        cpu.set_bp(0x11);
        assert_eq!(cpu.bp(), 0x11);
        // SP
        let mut cpu = CPU8086::new();
        cpu.set_sp(0x22);
        assert_eq!(cpu.sp(), 0x22);
        // DI
        let mut cpu = CPU8086::new();
        cpu.set_di(0x33);
        assert_eq!(cpu.di(), 0x33);
        // SI
        let mut cpu = CPU8086::new();
        cpu.set_si(0x44);
        assert_eq!(cpu.si(), 0x44);
    }
}