use std::collections::HashMap;
use crate::{Command, InstType, Register};

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

pub fn format_current_byte(asm_bytes: &Vec<u8>, current: usize) -> String {
    format!("ADDR {:02X}/{:02X} = {:02X} ({:08b}) {:02X} ({:08b})",
             current / 16, current % 16,
             asm_bytes[current], asm_bytes[current],
             asm_bytes[current + 1], asm_bytes[current + 1])
}